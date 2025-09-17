use std::fs;
use std::path::Path;

fn main() {
    // 创建模块目录
    create_module_dirs();
    
    // 读取目录中的所有文件
    let bin_dir = "src/bin";
    let entries = fs::read_dir(bin_dir).expect("无法读取目录");
    
    for entry in entries {
        let entry = entry.expect("无法获取目录项");
        let path = entry.path();
        
        // 确保是文件且扩展名为.rs
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            
            // 跳过main.rs
            if file_name == "main.rs" {
                continue;
            }
            
            // 读取文件内容
            let content = fs::read_to_string(&path).expect("无法读取文件");
            
            // 确定模块目录和新文件名
            let (module_dir, new_name) = determine_module_and_name(&content, &file_name);
            
            // 构建新路径
            let new_path = Path::new(bin_dir).join(&module_dir).join(&new_name);
            
            // 移动文件
            println!("移动: {} -> {}/{}", file_name, module_dir, new_name);
            fs::rename(&path, &new_path).expect("无法移动文件");
        }
    }
    
    println!("文件重组完成！");
}

fn create_module_dirs() {
    let module_dirs = [
        "src/bin/basic",
        "src/bin/traits",
        "src/bin/generics",
        "src/bin/enums",
        "src/bin/collections",
        "src/bin/option_result",
        "src/bin/structs",
        "src/bin/strings",
        "src/bin/functions",
    ];
    
    for dir in &module_dirs {
        fs::create_dir_all(dir).expect("无法创建目录");
    }
}

fn determine_module_and_name(content: &str, original_name: &str) -> (String, String) {
    // 根据文件内容确定模块和文件名
    if content.contains("trait Animal") || content.contains("trait Pet") {
        return ("traits".to_string(), "trait_inheritance.rs".to_string());
    }
    
    if content.contains("impl<T> Point<T>") || content.contains("struct Point") {
        return ("generics".to_string(), "generic_struct.rs".to_string());
    }
    
    if content.contains("fn match_option") {
        return ("option_result".to_string(), "match_option_example.rs".to_string());
    }
    
    if content.contains("Counter") && content.contains("HashMap") {
        return ("collections".to_string(), "counter_implementation.rs".to_string());
    }
    
    if content.contains("fn min<T: Ord>") {
        return ("generics".to_string(), "generic_function.rs".to_string());
    }
    
    if content.contains("enum MyEnum") {
        return ("enums".to_string(), "enum_memory_layout.rs".to_string());
    }
    
    if content.contains("name.pop()") && content.contains("while let") {
        return ("strings".to_string(), "string_pop_operations.rs".to_string());
    }
    
    if content.contains("hex_or_die_trying") {
        return ("option_result".to_string(), "option_result_handling.rs".to_string());
    }
    
    if content.contains("struct Race") {
        return ("structs".to_string(), "struct_impl_examples.rs".to_string());
    }
    
    if content.contains("box_memory_allocation") {
        return ("basic".to_string(), "box_memory_allocation.rs".to_string());
    }
    
    if content.contains("expression_tree_evaluation") {
        return ("collections".to_string(), "expression_tree_evaluation.rs".to_string());
    }
    
    if content.contains("comprehensive_rust_examples") {
        return ("basic".to_string(), "comprehensive_rust_examples.rs".to_string());
    }
    
    if content.contains("struct_impl_trait_examples") {
        return ("traits".to_string(), "struct_impl_trait_examples.rs".to_string());
    }
    
    // 默认处理其他文件
    if original_name.contains("small_demo_") {
        let id = original_name.replace("small_demo_", "").replace(".rs", "");
        return ("basic".to_string(), format!("small_example_{}.rs", id));
    }
    
    if original_name.contains("medium_demo_") {
        let id = original_name.replace("medium_demo_", "").replace(".rs", "");
        return ("basic".to_string(), format!("medium_example_{}.rs", id));
    }
    
    if original_name.contains("large_demo_") {
        let id = original_name.replace("large_demo_", "").replace(".rs", "");
        return ("basic".to_string(), format!("large_example_{}.rs", id));
    }
    
    // 默认情况
    ("basic".to_string(), original_name.to_string())
}