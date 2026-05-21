#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use unpdf::detect::is_pdf_bytes as detect_pdf_bytes;
use unpdf::model::Document;
use unpdf::render::{JsonFormat, RenderOptions};
use unpdf::{parse_bytes, render};

fn map_err(e: impl std::fmt::Display) -> Error {
    Error::from_reason(e.to_string())
}

fn parse(pdf: &[u8]) -> Result<Document> {
    parse_bytes(pdf).map_err(map_err)
}

fn with_doc<E: std::fmt::Display>(pdf: Buffer, f: impl FnOnce(&Document) -> std::result::Result<String, E>) -> Result<String> {
    f(&parse(pdf.as_ref())?).map_err(map_err)
}

#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[napi]
pub fn is_pdf_bytes(pdf: Buffer) -> bool {
    detect_pdf_bytes(pdf.as_ref())
}

#[napi]
pub fn to_markdown(pdf: Buffer) -> Result<String> {
    with_doc(pdf, |doc| render::to_markdown(doc, &RenderOptions::default()))
}

#[napi]
pub fn to_text(pdf: Buffer) -> Result<String> {
    with_doc(pdf, |doc| render::to_text(doc, &RenderOptions::default()))
}

#[napi]
pub fn to_json(pdf: Buffer, pretty: bool) -> Result<String> {
    let fmt = if pretty {
        JsonFormat::Pretty
    } else {
        JsonFormat::Compact
    };
    with_doc(pdf, |doc| render::to_json(doc, fmt))
}

#[napi]
pub fn get_info(pdf: Buffer) -> Result<String> {
    with_doc(pdf, |doc| {
        let m = &doc.metadata;
        serde_json::to_string(&serde_json::json!({
            "title": m.title,
            "author": m.author,
            "subject": m.subject,
            "keywords": m.keywords,
            "creator": m.creator,
            "producer": m.producer,
            "created": m.created,
            "modified": m.modified,
            "pdf_version": m.pdf_version,
            "page_count": doc.page_count(),
            "encrypted": m.encrypted,
            "resource_count": doc.resources.len(),
        }))
    })
}
