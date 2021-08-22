# FormValue
Implement impl Form&lt;CustomStruct> for serde_json::Value

# Example
    use from_value_derive::From;
    #[derive(From, Debug, Clone, Deserialize, Serialize)]
    pub struct Custom {
        name: String,
    }
    
    From派生宏生成如下：
    
    impl From<Custom> for Value{
    fn from(params: Custom) -> Self {
        let mut map = Map::new();
        map.insert("name".to_string(), Value::from(params.name));
        Value::Object(map)
    }