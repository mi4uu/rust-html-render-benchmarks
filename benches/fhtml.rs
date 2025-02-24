use std::fmt::Arguments;

fn main() {
    divan::main()
}

const SIZE: usize = 100;

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
    fhtml::format!{
        <table>
            {rows.iter().map(|cols| fhtml::format! {
                <tr>
                    {cols.iter().map(|col|  fhtml::format!{
                        <td>{*col}</td>
                    }).collect::<Vec<String>>().join("\n")}
                </tr>
            }.to_string()).collect::<Vec<String>>().join("\n")}
        </table>
    }
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
    let cls=["champion",""];
    fhtml::format! {
        <html>
            <head>
                <title>{teams.year}</title>
            </head>
            <body>
                <h1>{fhtml::format_args!("CSL {teams.year}")}</h1>
                <ul>
        { teams.teams.iter()
            .enumerate()
            .map(|(idx, team)|  fhtml::format!{
                <li class={match idx {
                        0=>"champion",
                        _=>""
                }}><b>
                 {&team.name} </b>":"
                  {team.score}</li> }
         ).collect::<Vec<String>>().join("\n")
        }
                </ul>
            </body>
        </html>
    }
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}