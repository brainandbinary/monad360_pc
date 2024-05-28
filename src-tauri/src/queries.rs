// src/queries.rs

use std::panic;

use crate::db::{get_database_path, DBError, Database, DB};
use rusqlite::{params, Connection, Result as RusqliteResult, Row};

#[derive(Debug, Clone)]
pub struct Score {
    pub id: Option<i32>,  /* db incremental id preimary key*/
    pub examType: String, /* 1=verbal, 2=quant */
    pub examId: String,
    pub score: i32, /* 130 to 170 */
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerbalAna {
    pub  id: Option<i32>,
    pub examId: String,
    pub tcCorrect: i32, /* percentage */
    pub rcCorrect: i32, /* percentage */
    pub scCorrect: i32, /* percentage */
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuantAna {
    pub examId: String,
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

pub fn insert_score(scr: &Score) -> Result<(), DBError> {
    let db = DB.lock().unwrap();

    let ch = db.as_ref().unwrap();

    let res: Result<usize, rusqlite::Error> = ch.connection.execute(
        "INSERT INTO mock_score (examType, examId, score ) VALUES (?1, ?2, ?3)",
        params![&scr.examType, &scr.examId, &scr.score],
    );

    match res {
        Ok(count) => println!("mock_score inserted {} row(s)", count),
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

    let mut stmt = ch
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
                    scCorrect: row.get(4)?
                })
            });

            match x {
                Ok(rs) => {
                    let mut z: Vec<VerbalAna> = rs
                        .into_iter()
                        .collect::<Result<Vec<VerbalAna>, _>>()?;


                     let _ = &z.sort_by(|a,b|{
                        a.id.unwrap().cmp(&b.id.unwrap())
                    });
                        

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
