use crate::{_Graph, Graph, external_apis::core::{ApiGraphType, Work}};
use reqwest::blocking::Client;
use std::{collections::{HashMap, HashSet}, error::Error};
use crate::external_apis::core::OpenAlexResponse;

fn openalex_make_request_search(search: &str, limit: usize, api_key: &str) -> Result<Vec<Work>, Box<dyn Error>> {
    let client = Client::new();
    let mut all_works: Vec<Work> = Vec::new();
    let mut cursor = String::from("*");

    loop {
        let url = format!(
            "https://api.openalex.org/works?search={}&per-page=200&cursor={}&api_key={}",
            search, cursor, api_key
        );

        let mut response = client
            .get(&url)
            .send()?
            .error_for_status()?
            .json::<OpenAlexResponse>()?;

        all_works.append(&mut response.results);

        if all_works.len() >= limit {
            all_works.truncate(limit);
            break;
        }

        match response.meta.next_cursor {
            Some(next) => cursor = next,
            None => break,
        }
    }

    return Ok(all_works);
}

fn openalex_make_batch_work_request(work_ids: HashSet<String>, api_key: &str) -> Result<Vec<Work>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut all_works: Vec<Work> = Vec::new();

    let clean_ids: Vec<String> = work_ids
        .into_iter()
        .map(|id| id.split('/').last().unwrap_or(&id).to_string())
        .collect();

    for chunk in clean_ids.chunks(50) {
        let ids_joined = chunk.join("|");

        let url = format!(
            "https://api.openalex.org/works?filter=openalex:{}&per-page=50&api_key={}",
            ids_joined, api_key
        );

        let mut response = client
            .get(&url)
            .send()?
            .error_for_status()?
            .json::<OpenAlexResponse>()?;

        all_works.append(&mut response.results);
    }

    return Ok(all_works);
}

fn openalex_coauthorship(search: &str, limit: usize, api_key: &str) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, limit, api_key).expect("Request to OpenAlex failed!");

    let mut unique_authors: HashSet<String> = HashSet::new();

    for work in &results {
        if let Some(authorships) = &work.authorships {

            for authorship in authorships {

                if let Some(author) = &authorship.author {
                    if let Some(name) = &author.display_name {
                        unique_authors.insert(name.clone());
                    }
                }
            }
        }
    }

    for author in &unique_authors {
        graph.add_node(author.clone());
    }

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();

    for work in &results {
        if let Some(authorships) = &work.authorships {

            let mut work_authors = Vec::new();
            for authorship in authorships {
                if let Some(author) = &authorship.author {
                    if let Some(name) = &author.display_name {
                        work_authors.push(name.clone());
                    }
                }
            }

            for i in 0..work_authors.len() {
                for j in (i + 1)..work_authors.len() {
                    let author_a = &work_authors[i];
                    let author_b = &work_authors[j];

                    let mut pair = vec![author_a.clone(), author_b.clone()];
                    pair.sort();

                    let key = (pair[0].clone(), pair[1].clone());

                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;
                }
            }
        }
    }

    for ((from, to), weight) in edges_weight {
        graph.create_connection(from, to, weight, Some(false));
    }

    return graph;

}

fn openalex_keyword_cooccurrence(search: &str, limit: usize, api_key: &str) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, limit, api_key).expect("Request to OpenAlex failed!");

    let mut unique_keywords: HashSet<String> = HashSet::new();

    for work in &results {
        if let Some(keywords) = &work.keywords {

            for keyword in keywords {
                if let Some(keyword_name) = &keyword.display_name {
                    unique_keywords.insert(keyword_name.clone());
                }
            }
        }
    }

    for keyword in &unique_keywords {
        graph.add_node(keyword.clone());
    }

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();

    for work in &results {
        if let Some(keywords) = &work.keywords {

            let mut work_keywords = Vec::new();
            for keyword in keywords {
                if let Some(keyword_name) = &keyword.display_name {
                    work_keywords.push(keyword_name.clone());
                }

            }

            for i in 0..work_keywords.len() {
                for j in (i + 1)..work_keywords.len() {
                    let keyword_a = &work_keywords[i];
                    let keyword_b = &work_keywords[j];

                    let mut pair = vec![keyword_a.clone(), keyword_b.clone()];
                    pair.sort();

                    let key = (pair[0].clone(), pair[1].clone());

                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;
                }
            }
        }
    }

    for ((from, to), weight) in edges_weight {
        graph.create_connection(from, to, weight, Some(false));
    }

    return graph;

}

fn openalex_cocitation(search: &str, limit: usize, api_key: &str) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, limit, api_key).expect("Request to OpenAlex failed!");

    let mut ids: HashSet<String> = HashSet::new();

    for work in &results {
        if let Some(referenced_works) = &work.referenced_works {
            for reference in referenced_works {
                ids.insert(reference.clone());
            }
        }
    }


    let cited_works_metadata = openalex_make_batch_work_request(ids, api_key)
        .expect("Batch request to OpenAlex failed!");

    let mut id_to_name: HashMap<String, String> = HashMap::new();

    for cited_work in cited_works_metadata {
        let clean_id = cited_work.id.split('/').last().unwrap_or(&cited_work.id).to_string();

        let name = cited_work.display_name.unwrap_or_else(|| clean_id.clone());

        id_to_name.insert(clean_id, name.clone());

        graph.add_node(name);
    }

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();

    for work in &results {
        if let Some(referenced_works) = &work.referenced_works {

            let mut work_references_names = Vec::new();

            for reference in referenced_works {
                let clean_id = reference.split('/').last().unwrap_or(&reference).to_string();
                if let Some(name) = id_to_name.get(&clean_id) {
                    work_references_names.push(name.clone());
                }
            }

            for i in 0..work_references_names.len() {
                for j in (i + 1)..work_references_names.len() {
                    let mut pair = vec![work_references_names[i].clone(), work_references_names[j].clone()];
                    pair.sort();

                    let key = (pair[0].clone(), pair[1].clone());
                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;
                }
            }
        }
    }

    for ((from, to), weight) in edges_weight {
        graph.create_connection(from, to, weight, Some(false));
    }

    return graph;
}

pub fn dispatch_openalex_graph_creation(search: &str, limit: usize, graph_type: ApiGraphType, api_key: &str) -> _Graph {
    let graph = match graph_type {
        ApiGraphType::Coauthorship => openalex_coauthorship(search, limit, api_key),
        ApiGraphType::KeywordCooccurrence => openalex_keyword_cooccurrence(search, limit, api_key),
        ApiGraphType::Cocitation => openalex_cocitation(search, limit, api_key)
    };

    return graph;
}
