use maud::html;

fn main() {
    divan::main()
}

const SIZE: usize = 1000;

#[divan::bench]
fn big_table(bencher: divan::Bencher) {
    let mut table = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        let mut inner = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            inner.push(i);
        }
        table.push(inner);
    }

    bencher.bench(|| big_table_render(&table))
}

fn big_table_render(rows: &Vec<Vec<usize>>) -> String {
    let page = html! {
        table {
            @for row in rows {
                tr {
                    @for col in row {
                        td { (col) }
                    }
                }
            }
        }
    };
    page.into_string()
}

#[divan::bench]
fn teams(bencher: divan::Bencher) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };

    bencher.bench(|| teams_render(&teams))
}

fn teams_render(teams: &Teams) -> String {
    let page = html! {
        html {
            head {
                title { (teams.year) }
            }
            body {
                h1 {
                    "CSL "
                    (teams.year)
                }
                ul {
                    @for (idx, team) in teams.teams.iter().enumerate() {
                        li.champion[idx == 0] {
                            b { (&team.name) }
                            ": "
                            (team.score)
                        }
                    }
                }
            }
        }
    };
    page.into_string()
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
