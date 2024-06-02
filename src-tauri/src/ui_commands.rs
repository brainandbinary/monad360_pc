use crate::db::DBError;
use crate::queries::{self, QuantAna, Score, VerbalAna, VerbalError};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn save_scores(
    examType: String,
    verbalScore: i32,
    quantScore: i32,
    verbalAna: VerbalAna,
    quantAna: QuantAna,
) -> Result<String, String> {
    let res = save_scores_result(examType, verbalScore, quantScore, verbalAna, quantAna);

    match res {
        Err(e) => Err(e.to_string()),
        Ok(r) => Ok(r),
    }
}

fn save_scores_result(
    examType: String,
    verbalScore: i32,
    quantScore: i32,
    verbalAna: VerbalAna,
    quantAna: QuantAna,
) -> Result<String, DBError> {
    let score: i32 = match examType.as_str() {
        "Verbal" => verbalScore,
        "Quant" => quantScore,
        _ => 0,
    };

    println!("{:?}", &verbalAna);

    let check = queries::get_score_one(&verbalAna.examId)?;

    if check.len() == 1 {
        match queries::update_score(&score, &verbalAna.examId) {
            Err(e) => Err(e),
            Ok(_) => match queries::update_verbal_ana(&verbalAna) {
                Err(e) => Err(e),
                Ok(_) => Ok("200".to_string()),
            },
        }
    } else {
        let obj: Score = Score {
            id: None,
            examType: examType.clone(),
            examId: verbalAna.examId.clone(),
            score: score,
        };
        match queries::insert_score(&obj) {
            Err(e) => Err(e),
            Ok(_) => {
                if &examType == "Verbal" {
                    match queries::insert_verbal_ana(&verbalAna) {
                        Err(r) => Err(r),
                        Ok(_) => Ok("200".to_string()),
                    }
                } else {
                    Ok("200".to_string())
                }
            }
        }
    }
}

#[tauri::command]
pub fn get_scores(isVerbal: bool) -> Vec<(i32, i32)> {
    queries::get_scores(isVerbal).unwrap()
}

#[tauri::command]
pub fn get_verbal_anas(limit: i32) -> Result<Vec<VerbalAna>, String> {
    match queries::get_verbal_anas(&limit) {
        Err(e) => Err(e.to_string()),
        Ok(r) => Ok(r),
    }
}

#[tauri::command]
pub fn get_verbal_errors(limit: i32) -> Result<(Vec<VerbalError>, Option<String>), String> {
    match queries::get_verbal_erros(&limit) {
        Err(e) => Err(e.to_string()),
        Ok(r) => Ok(r),
    }
}

#[tauri::command]
pub fn insert_or_update_verbal_errors(verbalError: VerbalError) -> Result<String, String> {
    match insert_or_update_verbal_errors_result(&verbalError) {
        Err(er) => Err(er.to_string()),
        Ok(_) =>  Ok("200".to_string()),
    }
}

fn insert_or_update_verbal_errors_result(verbal_error: &VerbalError) -> Result<String, DBError> {
    let check: Vec<VerbalError> = queries::get_verbal_error_one(&verbal_error.exam_id)?;

    if check.len() == 1 {
        match queries::update_verbal_error(&verbal_error) {
            Err(e) => Err(e),
            Ok(_) => Ok("200".to_string()),
        }
    } else {
        match queries::insert_verbal_error(&verbal_error) {
            Err(e) => Err(e),
            Ok(_) => Ok("200".to_string()),
        }
    }
}
