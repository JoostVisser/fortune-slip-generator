use anyhow::{anyhow, bail, Context, Result};
use lopdf::{Bookmark, Document, Object, ObjectId};
use std::{collections::BTreeMap, path::Path};

/// This code was copy-pasted from the lopdf crate.
/// Sadly, there's no high-level crate for PDF manipulation yet.
pub fn merge_pdf(input_paths: &[impl AsRef<Path>], output_path: impl AsRef<Path>) -> Result<()> {
    for input_path in input_paths {
        if !input_path.as_ref().is_file() {
            bail!("The path to the input PDF file is not a file");
        }
    }

    // Load all PDF files
    let documents = input_paths
        .iter()
        .map(Document::load)
        .map(|x| x.map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()?;

    // Define a starting max_id (will be used as start index for object_ids)
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages()
                .into_values()
                .map(|object_id| {
                    if !first {
                        let bookmark = Bookmark::new(
                            format!("Page_{}", pagenum),
                            [0.0, 0.0, 1.0],
                            0,
                            object_id,
                        );
                        document.add_bookmark(bookmark, None);
                        first = true;
                        pagenum += 1;
                    }

                    (object_id, doc.get_object(object_id).unwrap().to_owned())
                })
                .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // Catalog and Pages are mandatory
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
        // All other objects should be collected and inserted into the main Document
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages"
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            "Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" => {}     // Ignored, processed later and separately
            "Outlines" => {} // Ignored, not supported yet
            "Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" found abort
    if pages_object.is_none() {
        println!("Pages root not found.");
    }

    // Iter over all "Page" and collect with the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found abort
    if catalog_object.is_none() {
        println!("Catalog root not found.");
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                .into_keys()
                .map(Object::Reference)
                .collect::<Vec<_>>(),
        );

        document
            .objects
            .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
            .objects
            .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(ref mut dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    // Save the merged PDF
    document
        .save(&output_path)
        .with_context(|| format!("Cannot save to {:?}", &output_path.as_ref()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use lopdf::Document;
    use std::{fs::File, io::Write, path::PathBuf};
    use tempfile::tempdir;

    use crate::pdf::merge_pdf::merge_pdf;

    #[test]
    fn test_merge_pdf() {
        let tempdir = tempdir().unwrap();
        let input_path1 = PathBuf::from("test_utils/data/pdf_samples/dummy1.pdf");
        let input_path2 = PathBuf::from("test_utils/data/pdf_samples/dummy2.pdf");
        let output_path = tempdir.path().join("output.pdf");

        merge_pdf(&[&input_path1, &input_path2], &output_path).unwrap();

        let pdf = Document::load(&output_path).unwrap();
        let pages = pdf.get_pages();
        assert_eq!(pages.len(), 2);
    }

    #[test]
    fn test_merge_pdf_invalid_input_path() {
        // Create a temporary directory
        let tempdir = tempdir().unwrap();

        // Create a temporary input file that is not a PDF file
        let input_path = tempdir.path().join("input.txt");
        File::create(&input_path)
            .unwrap()
            .write_all(b"Hello, world!")
            .unwrap();

        // Merge the input files
        let result = merge_pdf(
            &[&input_path, &input_path],
            tempdir.path().join("output.pdf"),
        );

        // Verify that the merge operation failed with an error message
        assert!(result.is_err());
    }
}
