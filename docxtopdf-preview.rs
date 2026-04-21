let docx_bytes = include_bytes!("template.docx");
let pdf_bytes = office2pdf::convert_bytes(docx_bytes, office2pdf::config::Format::Docx, &Default::default())?;