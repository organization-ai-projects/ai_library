// Tests unitaires pour text.rs

use crate::data::preprocessing::text::*;

mod clean_texts_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let texts = vec![String::from("Hello, World!")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "hello world");
    }
    #[test]
    fn test_empty() {
        let texts = vec![String::from("")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "");
    }
    #[test]
    fn test_punctuation() {
        let texts = vec![String::from("C'est, vraiment! génial?")];
        let cleaned = clean_texts(&texts);
        assert_eq!(cleaned[0], "cest vraiment genial");
    }
}

mod tokenize_text_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let txt = "Hello world";
        assert_eq!(tokenize_text(txt), vec!["Hello", "world"]);
    }
    #[test]
    fn test_empty() {
        assert_eq!(tokenize_text("").len(), 0);
    }
}

mod embed_texts_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let texts = vec![String::from("hello world")];
        let embedded = embed_texts(&texts);
        assert_eq!(embedded[0], vec![10.0]);
    }
    #[test]
    fn test_empty() {
        let texts = vec![String::from("")];
        let embedded = embed_texts(&texts);
        assert_eq!(embedded[0], vec![0.0]);
    }
}

mod preprocess_texts_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let texts = vec![String::from("hello world")];
        let vecs = preprocess_texts(&texts);
        assert_eq!(vecs[0], vec![5.0, 5.0]);
    }
}
