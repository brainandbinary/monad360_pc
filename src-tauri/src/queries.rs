// src/queries.rs

use std::{panic, sync::Mutex};

use crate::db::{DBError, DB};
use rusqlite::params;

use lazy_static::lazy_static;
use uuid::Uuid;

/* to store examId on insert score, latter it can be accessed */
lazy_static! {
    static ref EXAM_ID: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Debug, Clone)]
pub struct Score {
    pub id: Option<i32>,  /* db incremental id preimary key*/
    pub examType: String, /* 1=verbal, 2=quant */
    pub examId: String,
    pub score: i32, /* 130 to 170 */
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerbalAna {
    pub id: Option<i32>,
    pub examId: String,
    pub tcCorrect: i32, /* percentage */
    pub rcCorrect: i32, /* percentage */
    pub scCorrect: i32, /* percentage */
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuantAna {
    pub examId: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerbalError {
    pub id: Option<i32>,
    pub exam_id: String,
    pub not_crossed: i32,
    pub idea_issue: i32,
    pub slow_passage: i32, 
    pub passage_check: i32, 
    pub time_killing: i32, 
}



pub fn create_verbal_ana_table() {
    let con = DB.lock().unwrap();

    let res = con.as_ref().unwrap().connection.execute(
        "CREATE TABLE IF NOT EXISTS verbal_ana (id INTEGER PRIMARY KEY, examId TEXT NOT NULL, tcCorrect INTEGER, rcCorrect INTEGER, scCorrect INTEGER);",
        [],
    );

    match res {
        Ok(_) => println!("verbal_ana Table created successfully"),
        Err(e) => eprintln!("Error creating table: {}", e),
    }
}

pub fn create_mock_score_table() {
    let con = DB.lock().unwrap();

    let res = con.as_ref().unwrap().connection.execute(
        "CREATE TABLE IF NOT EXISTS mock_score (id INTEGER PRIMARY KEY, examType TEXT NOT NULL , examId TEXT NOT NULL, score INTEGER);",
        [],
    );

    match res {
        Ok(_) => println!("mock_score Table created successfully"),
        Err(e) => eprintln!("Error creating table: {}", e),
    }
}


pub fn create_verbal_error_table() {
    let con = DB.lock().unwrap();

    let res = con.as_ref().unwrap().connection.execute(
        "CREATE TABLE IF NOT EXISTS verbal_error (id INTEGER PRIMARY KEY, exam_id TEXT NOT NULL, not_crossed INTEGER , idea_issue INTEGER, slow_passage INTEGER, passage_check INTEGER, time_killing INTEGER);",
        [],
    );

    

    match res {
        Ok(_) => println!("verbal_error Table created successfully"),
        Err(e) => eprintln!("Error creating table: {}", e),
    }
}


pub fn insert_score(scr: &Score) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "INSERT INTO mock_score (examType, examId, score ) VALUES (?1, ?2, ?3)",
        params![&scr.examType, &scr.examId, &scr.score],
    );

    match res {
        Ok(count) => {
            let mut exam_id_locked = EXAM_ID.lock().unwrap();
            *exam_id_locked = Some(scr.examId.clone());

            println!("mock_score inserted {} row(s)", count)
        }
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}

pub fn insert_verbal_error(scr: &VerbalError) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "INSERT INTO verbal_error (exam_id, not_crossed, idea_issue, slow_passage, passage_check, time_killing ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![ &scr.exam_id, &scr.not_crossed,&scr.idea_issue, &scr.slow_passage, &scr.passage_check, &scr.time_killing ],
    );

    match res {
        Ok(count) => {
        
            println!("verbal_error inserted {} row(s)", count)
        }
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}

pub fn insert_verbal_ana(scr: &VerbalAna) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "INSERT INTO verbal_ana (examId, tcCorrect, rcCorrect, scCorrect) VALUES (?1, ?2, ?3, ?4)",
        params![&scr.examId, &scr.tcCorrect, &scr.rcCorrect, &scr.scCorrect],
    );

    match res {
        Ok(count) => println!("verbal_ana inserted {} row(s)", count),
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}

pub fn update_score(score: &i32, exam_id: &String) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "UPDATE mock_score SET score = ?1 WHERE examId = ?2",
        params![score, exam_id],
    );

    let mut exam_id_locked = EXAM_ID.lock().unwrap();
            *exam_id_locked = Some(exam_id.clone());

    match res {
        Ok(count) => println!("mock_score total Updated {}", count),
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}

pub fn update_verbal_ana(scr: &VerbalAna) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "UPDATE verbal_ana SET tcCorrect = ?1 , rcCorrect = ?2 , scCorrect = ?3 WHERE examId = ?4",
        params![scr.tcCorrect, scr.rcCorrect, scr.scCorrect, scr.examId],
    );

    match res {
        Ok(count) => println!("Verbal ana total Updated {}", count),
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}

pub fn update_verbal_error(scr: &VerbalError) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "UPDATE verbal_error SET not_crossed=?1, idea_issue=?2, slow_passage=?3, passage_check=?4, time_killing=?5 WHERE exam_id = ?6",
        params![scr.not_crossed, scr.idea_issue, scr.slow_passage, scr.passage_check, scr.time_killing, scr.exam_id],
    );

    match res {
        Ok(count) => println!("Verbal error total Updated {}", count),
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    Ok(())
}


pub fn get_scores(is_verbal: bool) -> Result<Vec<(i32, i32)>, DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();


    let mut stmt = ch
        .connection
        .prepare("SELECT * FROM mock_score WHERE examType=?1");
    let examType = if is_verbal { "Verbal" } else { "Quant" };

    let res: Result<Vec<(i32, i32)>, _> = match stmt {
        Ok(mut st) => {
            let x = st.query_map([&examType], |row| {
                Ok(Score {
                    id: row.get(0)?,
                    examType: row.get(1)?,
                    examId: row.get(2)?,
                    score: row.get(3)?,
                })
            });

            
           
            match x {
                Ok(rs) => {
                    let z: Vec<(i32, i32)> = rs
                        .into_iter()
                        .map(move |e| {
                            println!("{:?}", e);

                            let score = e.as_ref().unwrap();
                            (score.id.unwrap(), score.clone().score)
                        })
                        .collect();

                    Ok(z)
                }
                Err(e) => {
                    panic!("Error retrieving rows: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Error retrieving rows: {}", e)
        }
    };

    res
}

pub fn get_verbal_anas(limit: &i32) -> Result<Vec<VerbalAna>, DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let stmt = ch
        .connection
        .prepare("select * from verbal_ana order by id desc limit ?1;");

    let res: Result<Vec<VerbalAna>, _> = match stmt {
        Ok(mut st) => {
            let x = st.query_map([limit], |row| {
                Ok(VerbalAna {
                    id: row.get(0)?,
                    examId: row.get(1)?,
                    tcCorrect: row.get(2)?,
                    rcCorrect: row.get(3)?,
                    scCorrect: row.get(4)?,
                })
            });

            match x {
                Ok(rs) => {
                    let mut z: Vec<VerbalAna> =
                        rs.into_iter().collect::<Result<Vec<VerbalAna>, _>>()?;

                    let _ = &z.sort_by(|a, b| a.id.unwrap().cmp(&b.id.unwrap()));

                    Ok(z)
                }
                Err(e) => {
                    panic!("Error retrieving rows: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Error retrieving rows: {}", e)
        }
    };

    res
}

pub fn get_scores_structs() -> Result<Vec<Score>, DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let mut stmt = ch
        .connection
        .prepare("SELECT * FROM mock_score WHERE examType=\"Verbal\"");

    let res: Result<Vec<Score>, _> = match stmt {
        Ok(mut st) => {
            let x = st.query_map([], |row| {
                Ok(Score {
                    id: row.get(0)?,
                    examType: row.get(1)?,
                    examId: row.get(2)?,
                    score: row.get(3)?,
                })
            });

            match x {
                Ok(rs) => {
                    let z: Vec<Score> = rs
                        .into_iter()
                        .map(move |e| {
                            println!("{:?}", e);

                            e.unwrap()
                        })
                        .collect();

                    Ok(z)
                }
                Err(e) => {
                    panic!("Error retrieving rows: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Error retrieving rows: {}", e)
        }
    };

    res
}


pub fn get_verbal_erros(limit: &i32) -> Result<(Vec<VerbalError>,Option<String>), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let stmt = ch
        .connection
        .prepare("select * from verbal_error order by id desc limit ?1;");

    let res: Result<(Vec<VerbalError>,Option<String>), _> = match stmt {
        Ok(mut st) => {
            let x = st.query_map([limit], |row| {
                Ok(VerbalError {
                    id: row.get(0)?,
                    exam_id: row.get(1)?,
                    not_crossed: row.get(2)?,
                    idea_issue: row.get(3)?,
                    slow_passage: row.get(4)?,
                    passage_check: row.get(5)?,
                    time_killing: row.get(6)?,
                })
            });

            match x {
                Ok(rs) => {
                    let mut z: Vec<VerbalError> =
                        rs.into_iter().collect::<Result<Vec<VerbalError>, _>>()?;

                    let _ = &z.sort_by(|a, b| a.id.unwrap().cmp(&b.id.unwrap()));

                    let exam_id_mutex: std::sync::MutexGuard<Option<String>> = EXAM_ID.lock().unwrap();
                    let r: Option<String> = exam_id_mutex.clone();

                    Ok((z,r))
                }
                Err(e) => {
                    panic!("Error retrieving rows: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Error retrieving rows: {}", e)
        }
    };

    res
}


pub fn get_score_one(exam_id: &String) -> Result<Vec<Score>, DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let mut stmt = ch
        .connection
        .prepare("SELECT * FROM mock_score WHERE examId= ?1")?;

    let rows = stmt.query_map([exam_id], |row| {
        Ok(Score {
            id: row.get(0)?,
            examType: row.get(1)?,
            examId: row.get(2)?,
            score: row.get(3)?,
        })
    })?;

    let scores: Vec<Score> = rows.collect::<Result<Vec<Score>, _>>()?;
    let score_clone = scores.clone();
    score_clone
        .into_iter()
        .for_each(|e| println!("hi----{:?}", e));

    Ok(scores)
}


pub fn get_verbal_error_one(exam_id: &String) -> Result<Vec<VerbalError>, DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let mut stmt = ch
        .connection
        .prepare("SELECT * FROM verbal_error WHERE exam_id= ?1")?;

    let rows = stmt.query_map([exam_id], |row| {
        Ok(VerbalError {
            id: row.get(0)?,
            exam_id: row.get(1)?,
            not_crossed: row.get(2)?,
            idea_issue: row.get(3)?,
            slow_passage: row.get(4)?,
            passage_check: row.get(5)?,
            time_killing: row.get(6)?,
        })
    })?;

    let scores: Vec<VerbalError> = rows.collect::<Result<Vec<VerbalError>, _>>()?;
    let score_clone: Vec<VerbalError> = scores.clone();
    score_clone
        .into_iter()
        .for_each(|e| println!("hi----{:?}", e));

    Ok(scores)
}
