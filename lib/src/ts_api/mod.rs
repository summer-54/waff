
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "id")]
    pub id:                i32, 
    #[serde(rename = "name")]
    pub name:              Box<str>,
    #[serde(rename = "inputFormat")]
    pub input_format:      Box<str>,
    #[serde(rename = "outputFormat")]
    pub output_format:     Box<str>,
    #[serde(rename = "tl")]
    pub tl:                i32,
    #[serde(rename = "rtl")]
    pub rtl:               i32,
    #[serde(rename = "ml")]
    pub ml:                i32,
    #[serde(rename = "statement")]
    pub statement:         Box<str>,
    #[serde(rename = "allowedLanguages")]
    pub allowed_languages: Box<str>,
    #[serde(rename = "samples")]
    pub samples:           Box<str>,
    #[serde(rename = "xp")]
    pub xp:                i32,
    #[serde(rename = "solved")]
    pub solved:            i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ContestWithTasks {
    #[serde(rename = "id")]
    pub id:           i32,
    /// Never serializes from api, need to be specified
    #[serde(skip)]
    pub group_id:     Option<i32>,
    #[serde(rename = "name")]
	pub name:         Box<str>,
    #[serde(rename = "dateFrom")]
	pub date_from:    i64,
    #[serde(rename = "dateUntil")]
	pub date_until:   i64,
    #[serde(rename = "isPublic")]
	pub is_public:    bool, 
    #[serde(rename = "needsReview")]
	pub needs_review: bool, 
    #[serde(rename = "upsolving")]
	pub upsolving:    bool, 
    #[serde(rename = "addedTime")]
	pub added_time:   i64,
    #[serde(rename = "markFormula")]
	pub mark_formula: Box<str>,
    #[serde(rename = "tasks")]
    pub tasks:        Box<[Task]>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Submission {
    #[serde(rename = "time")]
    pub time:        i64,
    #[serde(rename = "sourceCode")]
    pub source_code: Box<str>,
    #[serde(rename = "language")]
	pub language:    Box<str>,
    #[serde(rename = "taskId")]
	pub task_id:     i32,
    #[serde(rename = "contestId")]
	pub contest_id:  i32,
    #[serde(rename = "groupId")]
	pub group_id:    i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FullSubmission {
    #[serde(rename = "id")]
	pub id:          i32,
    #[serde(rename = "userId")]
	pub user_id:     i32,
    #[serde(rename = "verdict")]
	pub verdict:     i32,
    #[serde(rename = "test")]
	pub test:        i32,
    #[serde(rename = "comment")]
	pub comment:     Box<str>,
    #[serde(rename = "authorName")]
	pub author_name: Box<str>,
    #[serde(rename = "time")]
    pub time:        i64,
    #[serde(rename = "sourceCode")]
    pub source_code: Box<str>,
    #[serde(rename = "language")]
	pub language:    Box<str>,
    #[serde(rename = "taskId")]
	pub task_id:     i32,
    #[serde(rename = "taskName")]
	pub task_name:   Box<str>,
    #[serde(rename = "contestId")]
	pub contest_id:  i32,
}
