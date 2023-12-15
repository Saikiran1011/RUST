use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    job_id: f64,
    experience: String,
    qualifications: String,
    salary_range: String,
    location: String,
    country: String,
    latitude: f64,
    longitude: f64,
    work_type: String,
    company_size: u32,
    job_posting_date: String,
    preference: String,
    contact_person: String,
    contact: String,
    job_title: String,
    role: String,
    job_portal: String,
    job_description: String,
    benefits: String,
    skills: String,
    responsibilities: String,
    company: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let job1 = Job {
        job_id: 1.08984E+15,
        experience: "5 to 15 Years".to_string(),
        qualifications: "M.Tech".to_string(),
        salary_range: "$59K-$99K".to_string(),
        location: "Douglas".to_string(),
        country: "Isle of Man".to_string(),
        latitude: 54.2361,
        longitude: -4.5481,
        work_type: "Intern".to_string(),
        company_size: 26801,
        job_posting_date: "24-04-2022".to_string(),
        preference: "Female".to_string(),
        contact_person: "Brandon Cunningham".to_string(),
        contact: "001-381-930-7517x737".to_string(),
        job_title: "Digital Marketing Specialist".to_string(),
        role: "Social Media Manager".to_string(),
        job_portal: "Snagajob".to_string(),
        job_description: "Social Media Managers oversee an organization's social media presence. They create and schedule content, engage with followers, and analyze social media metrics to drive brand awareness and engagement.".to_string(),
        benefits: "{'Flexible Spending Accounts (FSAs), Relocation Assistance, Legal Assistance, Employee Recognition Programs, Financial Counseling'}".to_string(),
        skills: "Social media platforms (e.g., Facebook, Twitter, Instagram) Content creation and scheduling Social media analytics and insights Community engagement Paid social advertising".to_string(),
        responsibilities: "Manage and grow social media accounts, create engaging content, and interact with the online community. Develop social media content calendars and strategies. Monitor social media trends and engagement metrics.".to_string(),
        company: "Icahn Enterprises".to_string(),
    };

    let firebase = Firebase::new("https://rust-7c073-default-rtdb.firebaseio.com/").unwrap();

    let response_job1 = set_job(&firebase, &job1).await;

    let mut job = get_job(&firebase, response_job1.name.clone()).await;
    println!("{:?}", job);

    let jobs = get_jobs(&firebase).await;
    println!("{:?}", jobs);
}

async fn set_job(firebase_client: &Firebase, job: &Job) -> Response {
    let firebase = firebase_client.at("jobs");
    let _jobs = firebase.set::<Job>(&job).await;
    return string_to_response(&_jobs.unwrap().data);
}

async fn get_jobs(firebase_client: &Firebase) -> HashMap<String, Job> {
    let firebase = firebase_client.at("jobs");
    let jobs = firebase.get::<HashMap<String, Job>>().await;
    println!("{:?}", jobs);
    return jobs.unwrap();
}

async fn get_job(firebase_client: &Firebase, name: String) -> Job {
    let firebase = firebase_client.at("jobs").at(&name);
    let job = firebase.get::<Job>().await;
    return job.unwrap();
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}