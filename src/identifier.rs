#[derive(Debug, PartialEq)]
pub enum Identifier {
    Uuid(String),
    Isrc(String),
    Upc(String),
    Iswc(String),
    Ipi(String),
    PlatformUrl { platform: String, id: String },
}

pub fn detect(input: &str) -> Result<Identifier, String> {
    if is_uuid(input) {
        return Ok(Identifier::Uuid(input.to_string()));
    }

    if input.starts_with("http://") || input.starts_with("https://") {
        return parse_platform_url(input);
    }

    if is_isrc(input) {
        return Ok(Identifier::Isrc(input.to_string()));
    }

    if let Some(iswc) = normalize_iswc(input) {
        return Ok(Identifier::Iswc(iswc));
    }

    if is_ipi(input) {
        return Ok(Identifier::Ipi(input.to_string()));
    }

    if is_upc(input) {
        return Ok(Identifier::Upc(input.to_string()));
    }

    Err(format!(
        "Could not detect identifier type for '{}'. Expected: UUID, ISRC, ISWC, IPI, UPC, or platform URL.",
        input
    ))
}

fn is_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let expected_lens = [8, 4, 4, 4, 12];
    parts
        .iter()
        .zip(expected_lens.iter())
        .all(|(part, len)| part.len() == *len && part.chars().all(|c| c.is_ascii_hexdigit()))
}

fn is_isrc(s: &str) -> bool {
    if s.len() != 12 {
        return false;
    }
    let chars: Vec<char> = s.chars().collect();
    chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
        && chars[2..5].iter().all(|c| c.is_ascii_alphanumeric())
        && chars[5..].iter().all(|c| c.is_ascii_digit())
}

fn is_upc(s: &str) -> bool {
    (s.len() == 12 || s.len() == 13) && s.chars().all(|c| c.is_ascii_digit())
}

fn normalize_iswc(s: &str) -> Option<String> {
    let compact: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if compact.len() == 11
        && compact.starts_with('T')
        && compact[1..].chars().all(|c| c.is_ascii_digit())
    {
        Some(compact)
    } else {
        None
    }
}

fn is_ipi(s: &str) -> bool {
    s.len() == 11 && s.chars().all(|c| c.is_ascii_digit())
}

fn parse_platform_url(url: &str) -> Result<Identifier, String> {
    // Strip query params and trailing slashes for ID extraction
    let clean = url.split('?').next().unwrap_or(url).trim_end_matches('/');

    let (platform, id) = if url.contains("spotify.com") {
        // https://open.spotify.com/artist/ID or /track/ID or /album/ID
        let id = extract_last_path_segment(clean);
        ("spotify", id)
    } else if url.contains("music.apple.com") {
        // https://music.apple.com/us/artist/name/ID
        let id = extract_last_path_segment(clean);
        ("apple-music", id)
    } else if url.contains("youtube.com") {
        // https://www.youtube.com/watch?v=ID or /channel/ID
        if let Some(v) = url.split("v=").nth(1) {
            ("youtube", v.split('&').next().unwrap_or(v).to_string())
        } else {
            ("youtube", extract_last_path_segment(clean))
        }
    } else if url.contains("youtu.be") {
        // https://youtu.be/ID
        ("youtube", extract_last_path_segment(clean))
    } else if url.contains("deezer.com") {
        // https://www.deezer.com/track/ID or /artist/ID
        ("deezer", extract_last_path_segment(clean))
    } else if url.contains("soundcloud.com") {
        ("soundcloud", extract_last_path_segment(clean))
    } else if url.contains("tidal.com") {
        // https://tidal.com/browse/track/ID or /artist/ID
        ("tidal", extract_last_path_segment(clean))
    } else if url.contains("music.amazon") {
        ("amazon", extract_last_path_segment(clean))
    } else {
        return Err(format!("Unrecognized platform URL: {url}"));
    };

    if id.is_empty() {
        return Err(format!("Could not extract platform ID from URL: {url}"));
    }

    Ok(Identifier::PlatformUrl {
        platform: platform.to_string(),
        id,
    })
}

fn extract_last_path_segment(url: &str) -> String {
    url.rsplit('/').next().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_uuid() {
        let result = detect("a1b2c3d4-e5f6-7890-abcd-ef1234567890");
        assert_eq!(
            result.unwrap(),
            Identifier::Uuid("a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string())
        );
    }

    #[test]
    fn test_detect_isrc() {
        let result = detect("USUM72100001");
        assert_eq!(
            result.unwrap(),
            Identifier::Isrc("USUM72100001".to_string())
        );
    }

    #[test]
    fn test_detect_upc() {
        let result = detect("602435853161");
        assert_eq!(result.unwrap(), Identifier::Upc("602435853161".to_string()));
    }

    #[test]
    fn test_detect_spotify_url() {
        let result = detect("https://open.spotify.com/artist/3TVXtAsR1Inumwj472S9r4");
        match result.unwrap() {
            Identifier::PlatformUrl { platform, id } => {
                assert_eq!(platform, "spotify");
                assert_eq!(id, "3TVXtAsR1Inumwj472S9r4");
            }
            other => panic!("Expected PlatformUrl, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_spotify_url_with_query_params() {
        let result =
            detect("https://open.spotify.com/artist/2Lhs0asnFQiLuntn3s8p78?si=PTy9B7mvQWO");
        match result.unwrap() {
            Identifier::PlatformUrl { platform, id } => {
                assert_eq!(platform, "spotify");
                assert_eq!(id, "2Lhs0asnFQiLuntn3s8p78");
            }
            other => panic!("Expected PlatformUrl, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_iswc() {
        let result = detect("T9280410915");
        assert_eq!(result.unwrap(), Identifier::Iswc("T9280410915".to_string()));
    }

    #[test]
    fn test_detect_iswc_dotted() {
        let result = detect("T-928.041.091-5");
        assert_eq!(result.unwrap(), Identifier::Iswc("T9280410915".to_string()));
    }

    #[test]
    fn test_detect_ipi() {
        let result = detect("00832425062");
        assert_eq!(result.unwrap(), Identifier::Ipi("00832425062".to_string()));
    }

    #[test]
    fn test_ipi_not_upc() {
        // 11 digits should be IPI, not UPC
        let result = detect("12345678901");
        assert_eq!(result.unwrap(), Identifier::Ipi("12345678901".to_string()));
    }

    #[test]
    fn test_upc_still_works() {
        // 12 digits should still be UPC
        let result = detect("602435853161");
        assert_eq!(result.unwrap(), Identifier::Upc("602435853161".to_string()));
    }

    #[test]
    fn test_detect_unknown() {
        let result = detect("not-a-valid-identifier");
        assert!(result.is_err());
    }
}
