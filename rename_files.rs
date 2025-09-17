use std::fs;
use std::path::Path;

fn main() {
    let bin_dir = "src/bin";
    
    // 读取目录中的所有文件
    let entries = fs::read_dir(bin_dir).expect("无法读取目录");
    
    for entry in entries {
        let entry = entry.expect("无法获取目录项");
        let path = entry.path();
        
        // 确保是文件且扩展名为.rs
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            
            // 跳过已经具有描述性名称的文件
            if file_name == "main.rs" || file_name.contains("_") {
                continue;
            }
            
            // 读取文件内容
            let content = fs::read_to_string(&path).expect("无法读取文件");
            
            // 根据内容确定新文件名
            let new_name = determine_new_name(&content, &file_name);
            
            // 构建新路径
            let new_path = Path::new(bin_dir).join(new_name);
            
            // 重命名文件
            if path != new_path {
                println!("重命名: {} -> {}", file_name, new_path.file_name().unwrap().to_str().unwrap());
                fs::rename(&path, &new_path).expect("无法重命名文件");
            }
        }
    }
}

fn determine_new_name(content: &str, original_name: &str) -> String {
    // 根据文件内容确定描述性名称
    if content.contains("hex_or_die_trying") {
        return "option_result_examples.rs".to_string();
    }
    
    if content.contains("name.pop()") && content.contains("while let") {
        return "string_pop_while_let.rs".to_string();
    }
    
    if content.contains("enum Operation") && content.contains("enum Expression") && content.contains("Box<Expression>") {
        return "expression_tree_evaluation.rs".to_string();
    }
    
    if content.contains("Box::new") && content.contains("create_box") {
        return "box_memory_allocation.rs".to_string();
    }
    
    if content.contains("struct Race") && content.contains("impl Race") {
        return "struct_impl_trait_examples.rs".to_string();
    }
    
    if content.contains("format格式") && content.contains("std_trait") {
        return "comprehensive_rust_examples.rs".to_string();
    }
    
    // 处理剩余的demo文件
    if original_name.starts_with("demo") {
        // 根据文件大小和一些特征进行分类命名
        let lines_count = content.lines().count();
        if lines_count <= 10 {
            return format!("small_demo_{}.rs", &original_name[4..original_name.len()-3]);
        } else if lines_count <= 30 {
            return format!("medium_demo_{}.rs", &original_name[4..original_name.len()-3]);
        } else {
            return format!("large_demo_{}.rs", &original_name[4..original_name.len()-3]);
        }
    }
    
    // 默认情况下，保留原文件名
    original_name.to_string()
}