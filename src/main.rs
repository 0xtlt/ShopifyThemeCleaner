mod liquid;
mod template;
use crate::{
    liquid::{read_and_parse_liquid_file, LiquidFile},
    template::read_json_templates,
};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use tokio::{
    fs,
    task::{self},
};

#[derive(Parser, Debug)]
#[command(
    name = "ShopifyThemeCleaner",
    about = "A software for cleaning Shopify themes.",
    version = "0.1.0"
)]
struct Cli {
    #[arg(value_parser)]
    path: PathBuf,

    // --clean-all
    #[arg(long)]
    clean_all: bool,

    // --clean-snippets
    #[arg(long)]
    clean_snippets: bool,

    // --clean-sections
    #[arg(long)]
    clean_sections: bool,
}

lazy_static! {
    static ref REGEX_RENDER: Regex =
        Regex::new(r"\{\%[\s-]*render\s+'(?P<variable>[^\']+)'").unwrap();
    static ref REGEX_INCLUDE: Regex =
        Regex::new(r"\{\%[\s-]*include\s+'(?P<variable>[^\']+)'").unwrap();
    static ref REGEX_SECTION: Regex =
        Regex::new(r"\{\%[\s-]*section\s+'(?P<variable>[^\']+)'").unwrap();
    static ref REGEX_SCHEMA: Regex =
        Regex::new(r"\{\%\s*schema\s*\%\}(.*?)\{\%\s*endschema\s*\%\}").unwrap();
}

async fn read_and_parse_folder(path: PathBuf) -> Vec<task::JoinHandle<LiquidFile>> {
    let mut tasks = vec![];
    let mut dir = fs::read_dir(path).await.unwrap();

    while let Some(entry) = dir.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "liquid" {
            let path_str = path.into_os_string().into_string().unwrap();

            let task = task::spawn(async move { read_and_parse_liquid_file(path_str).await });
            tasks.push(task);
        }
    }

    tasks
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let base_path = cli.path;

    let should_clean_snippets = cli.clean_all;
    let should_clean_sections = cli.clean_all;

    println!("Processing path: {:?}", base_path);

    let metadata = fs::metadata(&base_path).await;

    if metadata.is_err() || !metadata.unwrap().is_dir() {
        println!("The path does not exist or is not a directory.");
        std::process::exit(1);
    }

    let mut tasks = vec![];

    // list all files in ./snippets
    let snippets_path = base_path.join("snippets");
    tasks.append(&mut read_and_parse_folder(snippets_path).await);

    // Attendre que toutes les tâches soient terminées et collecter les résultats
    let snippets: Vec<LiquidFile> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    // ----

    let mut tasks = vec![];

    // list all files in ./sections
    let sections_path = base_path.join("sections");
    tasks.append(&mut read_and_parse_folder(sections_path).await);

    let sections: Vec<LiquidFile> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    // ---
    // List all liquid files in ./layout
    let mut tasks = vec![];
    let layout_path = base_path.join("layout");

    tasks.append(&mut read_and_parse_folder(layout_path).await);

    let layouts: Vec<LiquidFile> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    // ---
    // List all liquid files in ./templates
    let mut tasks = vec![];
    let templates_path = base_path.join("templates");

    tasks.append(&mut read_and_parse_folder(templates_path).await);

    let templates: Vec<LiquidFile> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    // now in ./templates/customers
    let mut tasks = vec![];
    let customers_path = base_path.join("templates/customers");

    tasks.append(&mut read_and_parse_folder(customers_path).await);

    let customers: Vec<LiquidFile> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    // ---
    // For each .json templates, Make a TemplateJson struct
    let template_json_path = base_path.join("templates");
    let mut templates_json = read_json_templates(&template_json_path).await;

    let template_customers_path = base_path.join("templates/customers");
    templates_json.append(&mut read_json_templates(&template_customers_path).await);

    // ---

    // Collect all snippets used in sections
    let mut snippets_used_in_sections: Vec<String> = vec![];
    let mut sections_used: Vec<String> = vec![];

    for template in templates_json {
        for (key, value) in template.sections {
            if let Some(type_) = value._type {
                sections_used.push(type_);
            } else {
                sections_used.push(key);
            }
        }
    }

    for section in sections.iter() {
        snippets_used_in_sections.append(&mut section.snippet_renders.clone());
        sections_used.append(&mut section.section_renders.clone());
    }

    for layout in layouts {
        snippets_used_in_sections.append(&mut layout.snippet_renders.clone());
        sections_used.append(&mut layout.section_renders.clone());
    }

    for template in templates {
        snippets_used_in_sections.append(&mut template.snippet_renders.clone());
        sections_used.append(&mut template.section_renders.clone());
    }

    for customer in customers {
        snippets_used_in_sections.append(&mut customer.snippet_renders.clone());
        sections_used.append(&mut customer.section_renders.clone());
    }

    // check in snippets too
    for snippet in snippets.iter() {
        snippets_used_in_sections.append(&mut snippet.snippet_renders.clone());
        sections_used.append(&mut snippet.section_renders.clone());
    }

    // Now make a list of non-used snippets
    let mut non_used_snippets: Vec<String> = vec![];

    for snippet in snippets {
        let file_name = snippet.file_name.replace(".liquid", "");
        if !snippets_used_in_sections.contains(&file_name) {
            non_used_snippets.push(file_name);
        }
    }

    if should_clean_snippets {
        // Remove all non-used snippets
        for snippet in non_used_snippets {
            let path = base_path
                .join("snippets")
                .join(format!("{}.liquid", snippet));

            println!("Removing non-used snippet: {}", snippet);

            fs::remove_file(path)
                .await
                .expect("Failed to remove non-used snippet");
        }
    } else {
        // Just list all path of non-used snippets
        for snippet in non_used_snippets {
            println!("Non-used snippet: {}.liquid", snippet);
        }

        println!("You can use --clean-snippets to remove them automatically.");
    }

    // Now make a list of non-used sections
    let mut non_used_sections: Vec<String> = vec![];

    for section in sections.iter() {
        let file_name = section.file_name.replace(".liquid", "");
        if !sections_used.contains(&file_name) {
            non_used_sections.push(file_name);
        }
    }

    if should_clean_sections {
        // Remove all non-used sections
        for section in non_used_sections {
            let path = base_path
                .join("sections")
                .join(format!("{}.liquid", section));

            println!("Removing non-used section: {}", section);

            fs::remove_file(path)
                .await
                .expect("Failed to remove non-used section");
        }
    } else {
        // Just list all path of non-used sections
        for section in non_used_sections {
            println!("Non-used section: {}.liquid", section);
        }

        println!("You can use --clean-sections to remove them automatically.");
    }
}
