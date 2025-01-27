use std::collections::HashSet;

use crate::search_package::{search_package_lade, search_package_rade};

pub fn solve_dependencies(depends: &Vec<String>) -> Vec<String> {
    let mut solved: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();

    fn resolve(depends: &Vec<String>, solved: &mut Vec<String>, visited: &mut HashSet<String>) {
        for dependency in depends {
            if dependency.is_empty() {
                continue;
            }
            // すでに解決済みまたは訪問済みの場合はスキップ
            if visited.contains(dependency) {
                continue;
            }

            visited.insert(dependency.clone());

            // Ladeで検索
            if let Some(pkg_lade) = search_package_lade(dependency) {
                resolve(&pkg_lade.dependencies, solved, visited);
            } else if let Some(pkg_rade) = search_package_rade(dependency) {
                // Radeの依存関係をカンマで分割してリスト化
                let rade_dependencies: Vec<String> = pkg_rade
                    .dependencies
                    .split(',')
                    .map(|s| s.trim().to_string()) // 前後の空白を除去
                    .collect();

                resolve(&rade_dependencies, solved, visited);
            }

            // 依存関係を解決リストに追加
            if !solved.contains(dependency) {
                solved.push(dependency.clone());
            }
        }
    }

    resolve(depends, &mut solved, &mut visited);
    solved
}
