use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Lines, Write};
use std::ptr::write;
use std::time::Instant;

fn main() {
    let files = [
        "a_an_example",
        "b_better_start_small",
        "c_collaboration",
        "d_dense_schedule",
        "e_exceptional_skills",
        "f_find_great_mentors",
    ];

    let idx = 4;

    let input = "data/".to_string() + &files[idx] + ".in.txt";
    let input = &input;

    let file = File::open(input).expect("file not found");

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let line = lines.next().unwrap().unwrap();
    let nums = line.split(" ").collect::<Vec<_>>();
    let c: i64 = nums[0].parse().unwrap();
    let p: i64 = nums[1].parse().unwrap();

    let mut contributors = Vec::with_capacity(c as usize);

    for _ in 0..c {
        contributors.push(parse_contributor(&mut lines));
    }

    let mut projects = Vec::with_capacity(p as usize);
    for _ in 0..p {
        projects.push(parse_project(&mut lines));
    }

    println!("{contributors:?}");
    println!("{projects:?}");

    projects.sort_by_key(|project| project.best_before);
    // projects.sort_by_key(|project| project.roles.len());

    let mut skills_of_employee = HashMap::new();
    for contributor in &contributors {
        skills_of_employee.insert(contributor.name.clone(), HashMap::new());

        let skill_map = skills_of_employee.get_mut(&contributor.name).unwrap();
        for skill in contributor.skills.iter() {
            skill_map.insert(skill.lang.clone(), skill.level);
        }
    }

    let mut required_skills_by_project = HashMap::new();

    for project in &projects {
        required_skills_by_project.insert(project.name.clone(), HashMap::new());

        let required_skills = required_skills_by_project.get_mut(&project.name).unwrap();
        for skill in project.roles.iter() {
            required_skills.insert(skill.lang.clone(), skill.level);
        }
    }

    let mut latest_possible_day = 0;
    for project in &projects {
        latest_possible_day = std::cmp::max(latest_possible_day, project.best_before + project.score);
    }

    let mut employee_end_dates = HashMap::new();
    for contributor in &contributors {
        employee_end_dates.insert(contributor.name.clone(), -1);
    }

    let mut current_day = 0;

    let mut assns = Vec::new();
    let mut assigned_projects = HashSet::new();

    // try projects

    // ignore large projects
    projects = projects.into_iter().filter(|project| project.roles.len() <= 15).collect();

    println!("last day: {}", latest_possible_day);
    while current_day < latest_possible_day {
        println!("Checking day: {}", current_day);

        let mut min_project_duration = i64::MAX;
        let mut found_project = false;

        projects = projects.into_iter().filter(|project| !assigned_projects.contains(&project.name)).collect();

        for project in &projects {

            // if assigned_projects.contains(&project.name) {
            //     continue;
            // }
            //
            // if project.roles.len() > 15 {
            //     continue;
            // }

            // find if all skills are available
            // let mut available_employees_map = HashMap::new();

            // map langs -> levels -> employees of that level
            let mut available_employees_by_skill: HashMap<String, HashMap<i64, HashSet<String>>> = HashMap::new();

            for (employee, &end_date) in employee_end_dates.iter() {
                if end_date >= current_day {
                    continue;
                }

                let skills_of_employee = skills_of_employee.get_mut(employee).unwrap();

                for (skill, &level) in skills_of_employee.iter() {
                    available_employees_by_skill.entry(skill.clone()).or_insert(HashMap::new());

                    let level_to_employees = available_employees_by_skill.get_mut(skill).unwrap().entry(level).or_insert(HashSet::new());
                    level_to_employees.insert(employee.clone());
                }

            }

            /**

            mir hennd employees wo momentan frei sind

            wie findemer use, öb es projekt mit dene employees gfüllt werde chann

            */

            // println!("checking project {}", project.name);
            // println!("{:?}", available_employees_by_skill);

            let mut pre_fill_roles = Instant::now();

            // is there a way to fill project's roles with current employees?

            // let found_assn = fill_roles_bad(&project.roles, &available_employees_by_skill);
            // if let Some(assn) = found_assn {
            //     // println!("length of assn is: {}", assn.len());
            //     min_project_duration = if min_project_duration > project.days_to_complete {
            //         project.days_to_complete
            //     } else {
            //         min_project_duration
            //     };
            //     found_project = true;
            //     for assigned_employee in assn.iter() {
            //         let end_date = current_day + project.days_to_complete;
            //         *employee_end_dates.get_mut(assigned_employee).unwrap() = end_date;
            //     }
            //     assns.push(ProjectAssignment {
            //         project_name: project.name.clone(),
            //         roles_filled_by: assn,
            //     });
            //
            //     assigned_projects.insert(project.name.clone());
            // }
            let mut assn = Vec::new();
            let mut used_employees = HashSet::new();
            if fill_roles(&project.roles, &mut used_employees, &available_employees_by_skill, &mut assn) {
                // println!("length of assn is: {}", assn.len());
                min_project_duration = if min_project_duration > project.days_to_complete {
                    project.days_to_complete
                } else {
                    min_project_duration
                };
                found_project = true;
                for assigned_employee in assn.iter() {
                    let end_date = current_day + project.days_to_complete;
                    *employee_end_dates.get_mut(assigned_employee).unwrap() = end_date;
                }
                assns.push(ProjectAssignment {
                    project_name: project.name.clone(),
                    roles_filled_by: assn,
                });

                assigned_projects.insert(project.name.clone());

            }

            println!("took time for fill_roles: {:?}", Instant::now().duration_since(pre_fill_roles));
        }
        println!("min project duration: {}", min_project_duration);
        println!("found project: {}", found_project);

        if !found_project {
            // advance until the current employee which is done earliest is done

            // find employee
            let min_end_date = employee_end_dates.values().filter(|&&end_date| end_date >= current_day).min();
            if min_end_date.is_none() {
                break;
            }
            let min_end_date = min_end_date.unwrap();
            // for ()
            current_day = min_end_date + 1;
        } else {
            current_day += min_project_duration;
        }
    }

    let out_path = "out/".to_string() + files[idx] + ".txt";
    save_solution(&out_path, &assns);
    print_solution(&assns);

}

fn fill_roles_bad(roles: &[Skill], available_employees_by_skill: &HashMap<String, HashMap<i64, HashSet<String>>>) -> Option<Vec<String>> {
    let mut assn = Vec::new();

    let mut used_employees = HashSet::new();

    for skill in roles {
        if !available_employees_by_skill.contains_key(&skill.lang) {
            return None;
        }

        let level_to_employees = available_employees_by_skill.get(&skill.lang).unwrap();

        let mut found_employee = None;
        for (level, employees) in level_to_employees.iter() {
            if *level < skill.level {
                continue;
            }

            let employees = employees.iter().filter(|&employee| !used_employees.contains(employee)).collect::<Vec<_>>();
            if employees.len() == 0 {
                continue;
            }
            let employee = *employees.iter().next().unwrap();
            found_employee = Some(employee.clone());
        }
        if found_employee.is_none() {
            return None;
        }

        let found_employee = found_employee.unwrap();
        used_employees.insert(found_employee.clone());
        assn.push(found_employee);

    }

    Some(assn)
}


fn fill_roles(roles: &[Skill], used_employees: &mut HashSet<String>, available_employees_by_skill: &HashMap<String, HashMap<i64, HashSet<String>>>, current_assn: &mut Vec<String>) -> bool {
    if roles.len() == 0 {
        return true;
    }

    let current_skill = roles[0].clone();

    if !available_employees_by_skill.contains_key(&current_skill.lang) {
        return false;
    }
    let skill_to_employees = &available_employees_by_skill[&current_skill.lang];

    let mut available_employees = HashSet::new();
    for (level, employees) in skill_to_employees.iter() {
        if *level >= current_skill.level {
            available_employees.extend(employees.iter());
        }
    }

    for used_employee in used_employees.iter() {
        available_employees.remove(used_employee);
    }

    // if !skill_to_employees.contains_key(&current_skill.level) {
    //     return false;
    // }

    // let available_employees = &skill_to_employees[&current_skill.level];

    for &employee in available_employees.iter() {
        current_assn.push(employee.clone());
        used_employees.insert(employee.clone());

        if fill_roles(&roles[1..], used_employees, available_employees_by_skill, current_assn) {
            return true;
        }
        current_assn.pop();
        used_employees.remove(employee);


    }

    return false;
}

fn print_solution(assns: &[ProjectAssignment]) {
    println!("{}", assns.len());
    for assn in assns {
        println!("{}", assn.project_name);
        let mut contrib_iter = assn.roles_filled_by.iter();
        let first = contrib_iter.next().unwrap();
        print!("{}", first);
        for contrib in contrib_iter {
            print!(" {}", contrib);
        }
        println!();

    }
}

fn save_solution(file_path: &str, assns: &[ProjectAssignment]) {
    let mut file = File::create(file_path).unwrap();
    let mut writer = &mut file;
    // let mut writer = BufWriter::new(file);

    write!(writer, "{}\n", assns.len()).unwrap();
    for assn in assns {
        write!(writer, "{}\n", assn.project_name).unwrap();
        let mut contrib_iter = assn.roles_filled_by.iter();
        let first = contrib_iter.next().unwrap();

        write!(writer, "{}", first).unwrap();
        for contrib in contrib_iter {
            write!(writer, " {}", contrib).unwrap();
        }
        write!(writer, "\n").unwrap();
    }

    writer.flush();
}

struct ProjectAssignment {
    project_name: String,
    // which contributors are assigned to this project, in order
    roles_filled_by: Vec<String>,
}

fn parse_project(lines: &mut Lines<BufReader<File>>) -> Project {
    let line = lines.next().unwrap().unwrap();
    let parts = line.split(" ").collect::<Vec<_>>();

    let name = parts[0].to_string();
    let days_to_complete: i64 = parts[1].parse().unwrap();
    let score: i64 = parts[2].parse().unwrap();
    let best_before: i64 = parts[3].parse().unwrap();
    let num_roles: i64 = parts[4].parse().unwrap();

    let mut roles = Vec::with_capacity(num_roles as usize);
    for _ in 0..num_roles {
        roles.push(parse_skill(lines));
    }

    Project {
        name,
        days_to_complete,
        score,
        best_before,
        roles,
    }
}

#[derive(Debug, Clone)]
struct Project {
    name: String,
    days_to_complete: i64,
    score: i64,
    best_before: i64,

    roles: Vec<Skill>,
}

#[derive(Debug, Clone)]
struct Skill {
    lang: String,
    level: i64,
}

#[derive(Debug, Clone)]
struct Contributor {
    name: String,
    skills: Vec<Skill>,
}

fn parse_contributor(lines: &mut Lines<BufReader<File>>) -> Contributor {
    let first_line = lines.next().unwrap().unwrap();
    let parts = first_line.split(" ").collect::<Vec<_>>();
    let name = parts[0].to_string();
    let num_skills = parts[1].parse::<i64>().unwrap();

    let mut skills = Vec::with_capacity(num_skills as usize);
    for _ in 0..num_skills {
        skills.push(parse_skill(lines));
    }

    Contributor {
        skills,
        name,
    }
}

fn parse_skill(lines: &mut Lines<BufReader<File>>) -> Skill {
    let first_line = lines.next().unwrap().unwrap();
    let parts = first_line.split(" ").collect::<Vec<_>>();
    let lang = parts[0].to_string();
    let level = parts[1].parse::<i64>().unwrap();
    Skill {
        lang,
        level,
    }
}
