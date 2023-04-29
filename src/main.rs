use reqwest::Error;
use serde::{Deserialize, Serialize};
use clap::{App, Arg};
// Define a data structure to hold the JSON data
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    apiVersion: String,
    time: u32,
    params: Params,
    responses: Vec<Response>,
}
// Define the Response struct
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    time: u32,
    numResults: u32,
    results: Vec<ResultItem>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Params {
    species: String,
    limit: String,
}
// Define the ResultItem struct
#[derive(Serialize, Deserialize, Debug)]
struct ResultItem {
    accession: Vec<String>,
    // protein: serde_json::Value,
    gene: Vec<GeneWrapper>,
    keyword: Vec<Keyword>,
}


#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
struct GeneWrapper {    
    name: Vec<Gene>,
}
// Define the GeneInfo  
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]        
struct Gene {   
    value: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Keyword {
    value: String,
}


#[derive(Debug)]
struct ProteinAnnotation<'a> {
    accession: &'a Vec<String>,
    gene: Vec<&'a String>,
    keyword: Vec<&'a String>,
}

fn process_result_item<'a>(item: &'a ResultItem) -> ProteinAnnotation<'a> {
    let kwds: Vec<&'a String> = item.keyword.iter().map(|kwd| &kwd.value).collect();
    let gene_values: Vec<&'a String> = item
        .gene
        .iter()
        .flat_map(|wrapper| &wrapper.name)
        .map(|gene| &gene.value)
        .collect();

    ProteinAnnotation {
        accession: &item.accession,
        // protein: &item.protein,
        gene: gene_values,
        keyword: kwds,
    }
}

// In this version, we don't need to use Rc or Rc::new(), as the gene field is now a Vec<&'a String>. This simplifies the code and eliminates the unnecessary overhead of reference counting.



#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("Uniprot Adaptor Client")
        .version("1.0")
        .author("Mohammed Abdallah melsiddieg@gmail.com")
        .about("Retrieve protein annotations from CellBase websevices")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    // Replace this with the URL of the API you want to call
    let url = matches.value_of("url").unwrap();
    // let url = "https://ws.zettagenomics.com/cellbase/webservices/rest/v5/hsapiens/feature/ontology/search?assembly=grch38&count=false&limit=100&skip=0";
    // Make a GET request to the API
    let response = reqwest::get(url).await?;
    // Parse the JSON response
    let api_response: ApiResponse = response.json().await?;
    let results = &api_response.responses[0].results;
    // let processed_results: Vec<OntologyItem> = results.iter().map(process_result_item).collect();    
    // Use the parsed JSON data
    let processed_results: Vec<ProteinAnnotation> = results.iter().map(process_result_item).collect(); 
    println!("results: {:#?}", processed_results);

    Ok(())
}
