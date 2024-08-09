use {
    schemars::{schema::RootSchema, JsonSchema},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    serde_with::{json::JsonString, serde_as},
    std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    },
};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct KnowledgePanel {
    /// The name.
    #[schemars(length(min = 1, max = 100))]
    pub name: String,

    /// A blurb.
    #[schemars(length(min = 1, max = 400))]
    pub blurb: String,

    /// Factoids about the subject, format keys as proper nouns, include about six points.
    #[schemars(length(min = 1, max = 100))]
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct SearchResult {
    /// A website title, avoid repeating the subject.
    #[schemars(length(min = 1, max = 100))]
    pub title: String,

    /// An except from the website.
    #[schemars(length(min = 1, max = 300))]
    pub excerpt: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct SearchResults {
    /// A knowledge panel/overview/info cards for the searched topic.
    pub knowledge_panel: Option<KnowledgePanel>,

    /// Search engine results.
    #[schemars(length(min = 10, max = 20))]
    pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct CompletionRequest {
    prompt: String,
    json_schema: RootSchema,
    seed: u64,
}

#[serde_as]
#[derive(Deserialize)]
struct CompletionResponse<T: DeserializeOwned> {
    #[serde_as(as = "JsonString")]
    content: T,
}

static CACHE: LazyLock<Mutex<HashMap<String, SearchResults>>> =
    LazyLock::new(move || Mutex::new(HashMap::new()));

pub async fn request(query: &str) -> anyhow::Result<SearchResults> {
    if let Some(result) = CACHE.lock().unwrap().get(query) {
        return Ok(result.clone());
    }

    let json_schema = schemars::schema_for!(SearchResults);
    let schema = serde_json::to_string(&json_schema)?;

    println!("SCHEMA: {}", serde_json::to_string_pretty(&json_schema)?);

    let request = CompletionRequest {
        prompt: format!(include_str!("search.txt"), schema = schema, query = query),
        json_schema,
        seed: 0,
    };

    let result = reqwest::Client::new()
        .post("http://localhost:8080/completion")
        .json(&request)
        .send()
        .await?
        .json::<CompletionResponse<serde_json::Value>>()
        .await?
        .content;

    let result = serde_json::to_string_pretty(&result)?;

    println!("RESULT: {result}");

    let result = serde_json::from_str::<SearchResults>(&result)?;

    CACHE
        .lock()
        .unwrap()
        .insert(query.to_string(), result.clone());

    Ok(result)
}
