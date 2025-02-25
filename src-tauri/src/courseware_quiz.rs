use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::utils::{Dir, Load, Store, CONFIG_DIR};
#[derive(Serialize, Deserialize)]
pub struct Option {
    pub content: String,
    pub id: u64,
}
#[derive(Serialize, Deserialize)]
pub struct SingleSelectionQuestion {
    pub id: u64,
    pub description: String,
    pub options: Vec<Option>,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Question {
    SingleSelection(SingleSelectionQuestion),
}

#[derive(Serialize, Deserialize)]
pub struct CourseWareQuiz {
    pub id: u64,
    pub count: u64,
    pub questions: Vec<Question>,
    pub submitted_times: u64,
}

impl Dir for Vec<CourseWareQuiz> {
    fn dir() -> std::path::PathBuf {
        CONFIG_DIR.join("courseware_quiz.json")
    }
}

impl Load for Vec<CourseWareQuiz> {}
impl Store for Vec<CourseWareQuiz> {}

lazy_static!{
    pub static ref COURSEWARE_QUIZES: Vec<CourseWareQuiz> = Vec::load();
}

// https://courses.zju.edu.cn/api/courseware-quiz/quiz/191/subjects
// {
//   "subjects": [
//     {
//       "answer_number": 0,
//       "bloom_cognitive_domains": null,
//       "data": {

//       },
//       "description": "流水线 CPU 支持异常和中断实验的目的是什么？",
//       "difficulty_level": "none",
//       "id": 2597061,
//       "last_updated_at": "2025-02-13T08:55:28Z",
//       "note": null,
//       "options": [
//         {
//           "content": "掌握异常和中断的处理流程",
//           "id": 6945387,
//           "sort": 0,
//           "type": "text"
//         },
//         {
//           "content": "设计复杂的计算机系统",
//           "id": 6945388,
//           "sort": 1,
//           "type": "text"
//         },
//         {
//           "content": "研究数据存储方式",
//           "id": 6945389,
//           "sort": 2,
//           "type": "text"
//         },
//         {
//           "content": "探索新的编程语言",
//           "id": 6945390,
//           "sort": 3,
//           "type": "text"
//         }
//       ],
//       "parent_id": null,
//       "point": "0.0",
//       "settings": {
//         "case_sensitive": true,
//         "option_type": "text",
//         "options_layout": "horizontal",
//         "required": false,
//         "unordered": false,
//         "uploads": []
//       },
//       "sort": 0,
//       "sub_subjects": [],
//       "type": "single_selection"
//     },
//     {
//       "answer_number": 0,
//       "bloom_cognitive_domains": null,
//       "data": {

//       },
//       "description": "在发生异常/中断时，以下哪项不是硬件自动经历的状态转换？",
//       "difficulty_level": "none",
//       "id": 2597062,
//       "last_updated_at": "2025-02-13T08:55:28Z",
//       "note": null,
//       "options": [
//         {
//           "content": "异常指令的 PC 被保存在 mepc 中",
//           "id": 6945391,
//           "sort": 0,
//           "type": "text"
//         },
//         {
//           "content": "将控制状态寄存器 mstatus 中的 MIE 位置零",
//           "id": 6945392,
//           "sort": 1,
//           "type": "text"
//         },
//         {
//           "content": "把权限模式更改为 N",
//           "id": 6945393,
//           "sort": 2,
//           "type": "text"
//         },
//         {
//           "content": "根据异常来源设置 mcause",
//           "id": 6945394,
//           "sort": 3,
//           "type": "text"
//         }
//       ],
//       "parent_id": null,
//       "point": "0.0",
//       "settings": {
//         "case_sensitive": true,
//         "option_type": "text",
//         "options_layout": "horizontal",
//         "required": false,
//         "unordered": false,
//         "uploads": []
//       },
//       "sort": 1,
//       "sub_subjects": [],
//       "type": "single_selection"
//     },
//     {
//       "answer_number": 0,
//       "bloom_cognitive_domains": null,
//       "data": {

//       },
//       "description": "流水线 CPU 支持异常和中断实验的任务不包括以下哪项？",
//       "difficulty_level": "none",
//       "id": 2597063,
//       "last_updated_at": "2025-02-13T08:55:28Z",
//       "note": null,
//       "options": [
//         {
//           "content": "设计数据通路",
//           "id": 6945395,
//           "sort": 0,
//           "type": "text"
//         },
//         {
//           "content": "设计协处理器和控制器",
//           "id": 6945396,
//           "sort": 1,
//           "type": "text"
//         },
//         {
//           "content": "开发新的操作系统",
//           "id": 6945397,
//           "sort": 2,
//           "type": "text"
//         },
//         {
//           "content": "用程序验证流水线 CPU",
//           "id": 6945398,
//           "sort": 3,
//           "type": "text"
//         }
//       ],
//       "parent_id": null,
//       "point": "0.0",
//       "settings": {
//         "case_sensitive": true,
//         "option_type": "text",
//         "options_layout": "horizontal",
//         "required": false,
//         "unordered": false,
//         "uploads": []
//       },
//       "sort": 2,
//       "sub_subjects": [],
//       "type": "single_selection"
//     },
//     {
//       "answer_number": 0,
//       "bloom_cognitive_domains": null,
//       "data": {

//       },
//       "description": "以下哪个是机器模式状态寄存器（mstatus）在 RV32 中的作用？",
//       "difficulty_level": "none",
//       "id": 2597064,
//       "last_updated_at": "2025-02-13T08:55:28Z",
//       "note": null,
//       "options": [
//         {
//           "content": "处理异常和中断相关的状态",
//           "id": 6945399,
//           "sort": 0,
//           "type": "text"
//         },
//         {
//           "content": "存储计算结果",
//           "id": 6945400,
//           "sort": 1,
//           "type": "text"
//         },
//         {
//           "content": "控制指令执行顺序",
//           "id": 6945401,
//           "sort": 2,
//           "type": "text"
//         },
//         {
//           "content": "优化内存访问",
//           "id": 6945402,
//           "sort": 3,
//           "type": "text"
//         }
//       ],
//       "parent_id": null,
//       "point": "0.0",
//       "settings": {
//         "case_sensitive": true,
//         "option_type": "text",
//         "options_layout": "horizontal",
//         "required": false,
//         "unordered": false,
//         "uploads": []
//       },
//       "sort": 3,
//       "sub_subjects": [],
//       "type": "single_selection"
//     },
//     {
//       "answer_number": 0,
//       "bloom_cognitive_domains": null,
//       "data": {

//       },
//       "description": "在流水线 CPU 支持异常和中断的实验中，需要进行的检查点不包括以下哪项？",
//       "difficulty_level": "none",
//       "id": 2597065,
//       "last_updated_at": "2025-02-13T08:55:28Z",
//       "note": null,
//       "options": [
//         {
//           "content": "波形仿真的流水线 CPU 与验证程序",
//           "id": 6945403,
//           "sort": 0,
//           "type": "text"
//         },
//         {
//           "content": "FPGA 实现的流水线 CPU 与验证程序",
//           "id": 6945404,
//           "sort": 1,
//           "type": "text"
//         },
//         {
//           "content": "硬件性能测试",
//           "id": 6945405,
//           "sort": 2,
//           "type": "text"
//         },
//         {
//           "content": "以上都是",
//           "id": 6945406,
//           "sort": 3,
//           "type": "text"
//         }
//       ],
//       "parent_id": null,
//       "point": "0.0",
//       "settings": {
//         "case_sensitive": true,
//         "option_type": "text",
//         "options_layout": "horizontal",
//         "required": false,
//         "unordered": false,
//         "uploads": []
//       },
//       "sort": 4,
//       "sub_subjects": [],
//       "type": "single_selection"
//     }
//   ]
// }

// https://courses.zju.edu.cn/api/courseware-quiz/activity/963442/quizzes
// [
//   {
//     "id": 292,
//     "subjects_count": 10,
//     "submitted_times": 0,
//     "upload_reference_id": 13701656
//   }
// ]