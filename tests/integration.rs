#[cfg(test)]
mod tests {
    use crate::preprocess;
    
    #[test]
    fn test_preprocess() {
        let img = preprocess::preprocess_image("test.jpg", preprocess::PreprocessConfig { aggressive_denoise: false })
            .expect("Предобработка должна пройти");
        assert!(!img.is_empty());
    }
}
