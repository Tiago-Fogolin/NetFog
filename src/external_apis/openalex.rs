use crate::{_Graph, Graph, external_apis::core::{OpenAlexGraphType, Work}};
use reqwest::blocking::Client;
use std::{collections::{HashMap, HashSet}, error::Error};
use crate::external_apis::core::{OpenAlexResponse, AuthorReponse, KeyWordResponse};
use std::time::Instant;
use std::fs::File;

fn fetch_author_id(name: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.openalex.org/authors?search={}&api_key={}",
        name.replace(" ", "%20"), api_key
    );

    let response = client
        .get(&url)
        .send()?
        .error_for_status()?
        .json::<AuthorReponse>()?;



    if let Some(first_result) = response.results.as_ref().and_then(|vec| vec.first()) {
        if let Some(id) = &first_result.id {
            return Ok(id.clone());
        } else {
            panic!("Author not found!");
        }
    } else {
        panic!("Author not found!");
    }
}

fn fetch_keyword_id(name: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.openalex.org/keywords?search={}&api_key={}",
        name.replace(" ", "%20"), api_key
    );


    let response = client
        .get(&url)
        .send()?
        .error_for_status()?
        .json::<KeyWordResponse>()?;


    if let Some(first_result) = response.results.as_ref().and_then(|vec| vec.first()) {
        if let Some(id) = &first_result.id {
            return Ok(id.clone());
        } else {
            panic!("Keyword not found!");
        }
    } else {
        panic!("Keyword not found!");
    }
}

fn openalex_make_request_search(params: &str, api_key: &str, limit: Option<usize>) -> Result<Vec<Work>, Box<dyn Error>> {
    let client = Client::new();
    let mut all_works: Vec<Work> = Vec::new();
    let mut cursor = String::from("*");


    loop {
        let url = format!(
            "https://api.openalex.org/works?{}&cursor={}&api_key={}",
            params, cursor, api_key
        );

        let mut response = client
            .get(&url)
            .send()?
            .error_for_status()?
            .json::<OpenAlexResponse>()?;

        all_works.append(&mut response.results);

        if !limit.is_none() && all_works.len() >= limit.unwrap() {
            all_works.truncate(limit.unwrap());
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

fn openalex_coauthorship(search: &str, api_key: &str, limit: Option<usize>, min_weight: Option<f32>, save_json_path: Option<&str>) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, api_key, limit).expect("Request to OpenAlex failed!");

    let start = Instant::now();
    let mut unique_authors: HashSet<String> = HashSet::new();

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();

    // Max edge weight by author
    let mut max_edge_weight: HashMap<String, f32> = HashMap::new();

    for work in &results {
        if let Some(authorships) = &work.authorships {

            let mut work_authors = Vec::new();
            for authorship in authorships {
                if let Some(author) = &authorship.author {
                    if let Some(name) = &author.display_name {
                        work_authors.push(name.clone());
                        unique_authors.insert(name.clone());
                    }
                }
            }

            for i in 0..work_authors.len() {
                for j in (i + 1)..work_authors.len() {
                    let author_a = &work_authors[i];
                    let author_b = &work_authors[j];


                    let key = if author_a < author_b {
                        (author_a.clone(), author_b.clone())
                    } else {
                        (author_b.clone(), author_a.clone())
                    };

                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;

                    max_edge_weight.entry(author_a.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);

                    max_edge_weight.entry(author_b.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);
                }
            }
        }
    }
    let mut valid_nodes: HashSet<String> = HashSet::new();

    for author in &unique_authors {
        let mut place = true;

        if !min_weight.is_none() {
            let max_weight = *max_edge_weight.get(author).unwrap_or(&0.);
            if max_weight < min_weight.unwrap() {
                place = false;
            }
        }

        if place {
            graph.add_node(author.clone());
            valid_nodes.insert(author.clone());
        }
    }

    for ((from, to), weight) in edges_weight {
        if min_weight.is_none() {
            graph.create_connection(from, to, weight, Some(false));
        }
        else if valid_nodes.contains(&from) && valid_nodes.contains(&to) && weight >= min_weight.unwrap() {
            graph.create_connection(from, to, weight, Some(false));
        }
    }

    let duration = start.elapsed();

    graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

    if let Some(path) = save_json_path {
        match File::create(path) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &results) {
                    eprintln!("Falha ao escrever o JSON no arquivo: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Falha ao criar o arquivo JSON em {}: {}", path, e);
            }
        }
    }

    return graph;

}

fn openalex_keyword_cooccurrence(search: &str, api_key: &str, limit: Option<usize>, min_weight: Option<f32>, save_json_path: Option<&str>) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, api_key, limit).expect("Request to OpenAlex failed!");

    let start = Instant::now();

    let mut unique_keywords: HashSet<String> = HashSet::new();

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();

    // Max edge weight by keyword
    let mut max_edge_weight: HashMap<String, f32> = HashMap::new();


    for work in &results {
        if let Some(keywords) = &work.keywords {

            let mut work_keywords = Vec::new();
            for keyword in keywords {
                if let Some(keyword_name) = &keyword.display_name {
                    work_keywords.push(keyword_name.clone());
                    unique_keywords.insert(keyword_name.clone());
                }

            }

            for i in 0..work_keywords.len() {
                for j in (i + 1)..work_keywords.len() {
                    let keyword_a = &work_keywords[i];
                    let keyword_b = &work_keywords[j];

                    let key = if keyword_a < keyword_b {
                        (keyword_a.clone(), keyword_b.clone())
                    } else {
                        (keyword_b.clone(), keyword_a.clone())
                    };

                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;

                    max_edge_weight.entry(keyword_a.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);

                    max_edge_weight.entry(keyword_b.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);
                }
            }
        }
    }

    let mut valid_nodes: HashSet<String> = HashSet::new();
    for keyword in &unique_keywords {
        let mut place = true;

        if !min_weight.is_none() {
            let max_weight = *max_edge_weight.get(keyword).unwrap_or(&0.);
            if max_weight < min_weight.unwrap() {
                place = false;
            }
        }

        if place {
            graph.add_node(keyword.clone());
            valid_nodes.insert(keyword.clone());
        }
    }

    for ((from, to), weight) in edges_weight {
        if min_weight.is_none() {
            graph.create_connection(from, to, weight, Some(false));
        }
        else if valid_nodes.contains(&from) && valid_nodes.contains(&to) && weight >= min_weight.unwrap() {
            graph.create_connection(from, to, weight, Some(false));
        }
    }

    let duration = start.elapsed();

    graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

    if let Some(path) = save_json_path {
        match File::create(path) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &results) {
                    eprintln!("Falha ao escrever o JSON no arquivo: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Falha ao criar o arquivo JSON em {}: {}", path, e);
            }
        }
    }

    return graph;

}

pub enum CocitationType {
    Work,
    Author,
}

fn openalex_cocitation(
    search: &str,
    api_key: &str,
    limit: Option<usize>,
    min_weight: Option<f32>,
    co_type: CocitationType,
    save_json_path: Option<&str>
) -> _Graph {
    let mut graph = _Graph::default();
    let results = openalex_make_request_search(search, api_key, limit).expect("Request to OpenAlex failed!");

    let start_first_process = Instant::now();
    let mut ids: HashSet<String> = HashSet::new();

    for work in &results {
        if let Some(referenced_works) = &work.referenced_works {
            for reference in referenced_works {
                ids.insert(reference.clone());
            }
        }
    }

    let mut build_time = start_first_process.elapsed();

    let cited_works_metadata = openalex_make_batch_work_request(ids, api_key)
        .expect("Batch request to OpenAlex failed!");

    let start_second_process = Instant::now();


    let mut id_to_entities: HashMap<String, Vec<String>> = HashMap::new();

    for cited_work in cited_works_metadata {
        let clean_id = cited_work.id.split('/').last().unwrap_or(&cited_work.id).to_string();

        let entities = match co_type {
            CocitationType::Work => {
                let name = cited_work.display_name.clone().unwrap_or_else(|| clean_id.clone());
                vec![name]
            },
            CocitationType::Author => {
                let mut authors = Vec::new();
                if let Some(authorships) = &cited_work.authorships {
                    for authorship in authorships {
                        if let Some(author) = &authorship.author {
                            if let Some(name) = &author.display_name {
                                authors.push(name.clone());

                            }
                        }
                    }
                }
                authors
            }
        };

        id_to_entities.insert(clean_id, entities);
    }

    let mut edges_weight: HashMap<(String, String), f32> = HashMap::new();
    let mut max_edge_weight: HashMap<String, f32> = HashMap::new();

    for work in &results {
        if let Some(referenced_works) = &work.referenced_works {

            let mut entities_in_this_work = Vec::new();

            for reference in referenced_works {
                let clean_id = reference.split('/').last().unwrap_or(&reference);
                if let Some(entities) = id_to_entities.get(clean_id) {
                    for entity in entities {
                        entities_in_this_work.push(entity.clone());
                    }
                }
            }

            entities_in_this_work.sort();
            entities_in_this_work.dedup();

            for i in 0..entities_in_this_work.len() {
                for j in (i + 1)..entities_in_this_work.len() {
                    let entity_a = &entities_in_this_work[i];
                    let entity_b = &entities_in_this_work[j];

                    let key = if entity_a < entity_b {
                        (entity_a.clone(), entity_b.clone())
                    } else {
                        (entity_b.clone(), entity_a.clone())
                    };

                    let counter = edges_weight.entry(key).or_insert(0.0);
                    *counter += 1.0;

                    max_edge_weight.entry(entity_a.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);

                    max_edge_weight.entry(entity_b.clone())
                        .and_modify(|w| *w = w.max(*counter))
                        .or_insert(*counter);
                }
            }
        }
    }

    let mut valid_nodes: HashSet<String> = HashSet::new();

    for entities in id_to_entities.values() {
        for name in entities {
            let mut place = true;

            if let Some(min_w) = min_weight {
                let max_weight = *max_edge_weight.get(name).unwrap_or(&0.);
                if max_weight < min_w {
                    place = false;
                }
            }

            if valid_nodes.contains(name) {
                place = false;
            }

            if place {
                graph.add_node(name.clone());
                valid_nodes.insert(name.clone());
            }
        }
    }

    for ((from, to), weight) in edges_weight {
        if min_weight.is_none() {
            graph.create_connection(from, to, weight, Some(false));
        } else if valid_nodes.contains(&from) && valid_nodes.contains(&to) && weight >= min_weight.unwrap() {
            graph.create_connection(from, to, weight, Some(false));
        }
    }

    let duration_second_process = start_second_process.elapsed();
    build_time += duration_second_process;
    graph.build_time_ms = Some(build_time.as_secs_f64() * 1000.0);

    if let Some(path) = save_json_path {
        match File::create(path) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &results) {
                    eprintln!("Falha ao escrever o JSON no arquivo: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Falha ao criar o arquivo JSON em {}: {}", path, e);
            }
        }
    }

    return graph;
}

pub fn dispatch_openalex_graph_creation(
    search: Option<&str>,
    author: Option<&str>,
    author_id: Option<&str>,
    author_orcid: Option<&str>,
    keyword: Option<&str>,
    graph_type: OpenAlexGraphType,
    api_key: &str,
    limit: Option<usize>,
    min_weight: Option<f32>,
    save_json_path: Option<&str>
) -> _Graph {
    let mut filters: Vec<String> = Vec::new();

    if let Some(name) = author {
        let author_id = fetch_author_id(name, api_key).expect("Author not found!");
        filters.push(format!("author.id:{}", author_id));
    }

    if let Some(auth_id) = author_id {
        filters.push(format!("author.id:{}", auth_id));
    }

    if let Some(auth_orcid) = author_orcid {
        filters.push(format!("author.orcid:{}", auth_orcid));
    }

    if let Some(kw) = keyword {
        let kw_id = fetch_keyword_id(kw, api_key).expect("Keyword not found!");
        filters.push(format!("keywords.id:{}", kw_id));
    }

    if filters.is_empty() && search.is_none() {
        panic!("At least one filter must be filled (search, author or keyword).");
    }

    let mut query_params: Vec<String> = Vec::new();

    if let Some(srch) = search {
        query_params.push(format!("search={}", srch.replace(" ", "%20")));
    }

    if !filters.is_empty() {
        let joined_filters = filters.join(",");
        query_params.push(format!("filter={}", joined_filters));
    }

    query_params.push("per-page=200".to_string());

    let params = query_params.join("&");

    let graph = match graph_type {
        OpenAlexGraphType::Coauthorship => openalex_coauthorship(&params, api_key, limit, min_weight, save_json_path),
        OpenAlexGraphType::KeywordCooccurrence => openalex_keyword_cooccurrence(&params, api_key, limit, min_weight, save_json_path),
        OpenAlexGraphType::WorkCocitation => openalex_cocitation(&params, api_key, limit, min_weight, CocitationType::Work, save_json_path),
        OpenAlexGraphType::AuthorCocitation => openalex_cocitation(&params, api_key, limit, min_weight, CocitationType::Author, save_json_path)
    };

    return graph;
}
