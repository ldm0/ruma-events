macro_rules! serde_eq {
    ($de:literal, $se:expr) => {
        let mut val = $se;
        assert_eq!($de, to_string(&val).unwrap());
        val = from_str($de).unwrap();
        assert_eq!(val, $se);
    };
}
