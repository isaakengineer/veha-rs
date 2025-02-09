	let file_content = fs::read_to_string("./beispiel/buch/eintrag.xhtml").expect("Unable to read file");
    let document = xml::parse(file_content.as_str()).expect("Failed to parse XML");

    // Find all <vorlage> elements
    let vorlages: Vec<Element> = document.descendants().filter(|n| n.is_element() && n.name() == "vorlage").collect();

    for vorlage in vorlages {
	    if let Some(template_name) = vorlage.attribute("name") {
	        let content = fs::read_to_string(&format!("./beispiel/vorlagen/{}.xhtml", template_name)).expect("Unable to read file");
	        let template_document = xml::parse(content.as_str()).expect("Failed to parse XML");
			if let Some(slot) = vorlage.first_child_element("slot") {
            	// Read the corresponding template file based on the name attribute
             	slot.set_children(vorlage..into_iter().collect());
	        }
	        // Replace the content of <slot></slot> with everything in between the "<vorlage>" tag

	    }

    }
