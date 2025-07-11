#[cfg(test)]
mod tests {
    use super::clean_texts;
    use super::embed_texts;
    use super::preprocess_texts;

    #[test]
    fn test_preprocess_texts_basic() {
        let texts = vec![String::from("hello world")];
        let vecs = preprocess_texts(&texts);
        assert_eq!(vecs[0], vec![5.0, 5.0]);
    }

    #[test]
    fn test_clean_texts_basic() {
        let texts = vec![String::from("Hello, World!")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "hello world");
    }

    #[test]
    fn test_clean_texts_empty() {
        let texts = vec![String::from("")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "");
    }

    #[test]
    fn test_clean_texts_punctuation() {
        let texts = vec![String::from("C'est, vraiment! génial?")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "cest vraiment genial");
    }

    #[test]
    fn test_embed_texts_basic() {
        let texts = vec![String::from("hello world")];
        let embedded = embed_texts(&texts);
        assert_eq!(embedded[0], vec![10.0]);
    }

    #[test]
    fn test_embed_texts_empty() {
        let texts = vec![String::from("")];
        let embedded = embed_texts(&texts);
        assert_eq!(embedded[0], vec![0.0]);
    }
}
