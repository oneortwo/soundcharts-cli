#[derive(Debug, PartialEq)]
pub enum Identifier {
    Uuid(String),
    Isrc(String),
    Upc(String),
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

    if is_upc(input) {
        return Ok(Identifier::Upc(input.to_string()));
    }

    Err(format!(
        "Could not detect identifier type for '{}'. Expected: UUID, ISRC, UPC, or platform URL.",
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

fn parse_platform_url(url: &str) -> Result<Identifier, String> {
    let platform = if url.contains("spotify.com") {
        "spotify"
    } else if url.contains("music.apple.com") {
        "apple-music"
    } else if url.contains("youtube.com") || url.contains("youtu.be") {
        "youtube"
    } else if url.contains("deezer.com") {
        "deezer"
    } else if url.contains("soundcloud.com") {
        "soundcloud"
    } else if url.contains("tidal.com") {
        "tidal"
    } else if url.contains("music.amazon") {
        "amazon"
    } else {
        return Err(format!("Unrecognized platform URL: {url}"));
    };

    Ok(Identifier::PlatformUrl {
        platform: platform.to_string(),
        id: url.to_string(),
    })
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
        assert_eq!(
            result.unwrap(),
            Identifier::Upc("602435853161".to_string())
        );
    }

    #[test]
    fn test_detect_spotify_url() {
        let result = detect("https://open.spotify.com/artist/3TVXtAsR1Inumwj472S9r4");
        match result.unwrap() {
            Identifier::PlatformUrl { platform, .. } => assert_eq!(platform, "spotify"),
            other => panic!("Expected PlatformUrl, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_unknown() {
        let result = detect("not-a-valid-identifier");
        assert!(result.is_err());
    }
}
