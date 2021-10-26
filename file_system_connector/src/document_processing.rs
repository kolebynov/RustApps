use std::{sync::mpsc::{self, Sender, SyncSender}};

use log::{info, error};

use crate::{common::Document, indexing_api::{AddDocumentRequest, AddDocumentsRequest, IndexingApiClient}, settings::{self, IndexingApiSettings}};

pub struct DocumentProcessor {
    tx: SyncSender<Document>,
}

impl DocumentProcessor {
    pub fn new(indexing_api: IndexingApiClient, settings: IndexingApiSettings) -> DocumentProcessor {
        let (tx, rx) = mpsc::sync_channel(100);
        tokio::spawn(async move {
            let mut local_queue = Vec::new();
            let mut sent_documents: i64 = 0;

            loop {
                {
                    if let Result::Ok(document) = rx.recv() {
                        local_queue.push(document);
                    } else {
                        return;
                    }
                }

                if local_queue.len() < 50 {
                    continue;
                }

                info!("Sending {} documents to IAPI", local_queue.len());
                let add_result = indexing_api.add_documents(AddDocumentsRequest {
                    connector_tag: "TestConnector".to_string(),
                    connector_version: "1.0.0".to_string(),
                    items: local_queue.iter().map(|d: &Document| AddDocumentRequest {
                        kb_id: settings.kb_id,
                        subset_id: "Test".to_string(),
                        language: 32767,
                        uri: d.uri.clone(),
                        open_url: d.uri.clone(),
                    }).collect(),
                }).await;

                if let Result::Err(err) = add_result {
                    error!("Failed to send documents to IAPI: {}", err);
                    panic!("Failed to send documents to IAPI: {}", err);
                }

                sent_documents += local_queue.len() as i64;
                info!("Sent documents: {}", sent_documents);
                
                local_queue.clear();
            }
        });

        DocumentProcessor { tx }
    }

    pub fn process_document(&self, document: Document) -> Result<(), Box<dyn std::error::Error>> {
        self.tx.send(document)?;

        Ok(())
    }
}