use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ToolType {
    Git,
    Docker,
    Kubernetes,
    Tmux,
    Bash,
}

impl ToolType {
    fn all() -> Vec<ToolType> {
        vec![
            ToolType::Git,
            ToolType::Docker,
            ToolType::Kubernetes,
            ToolType::Tmux,
            ToolType::Bash,
        ]
    }

    fn translation_key(&self) -> &'static str {
        match self {
            ToolType::Git => "cheatsheet_viewer.tool_git",
            ToolType::Docker => "cheatsheet_viewer.tool_docker",
            ToolType::Kubernetes => "cheatsheet_viewer.tool_kubernetes",
            ToolType::Tmux => "cheatsheet_viewer.tool_tmux",
            ToolType::Bash => "cheatsheet_viewer.tool_bash",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToolType::Git => "Git",
            ToolType::Docker => "Dkr",
            ToolType::Kubernetes => "K8s",
            ToolType::Tmux => "Tmx",
            ToolType::Bash => "Sh",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CheatsheetCategory {
    All,
    Basic,
    Branch,
    Remote,
    Advanced,
    Container,
    Image,
    Network,
    Volume,
    Pod,
    Service,
    Deployment,
    Config,
    Session,
    Window,
    Pane,
    FileOps,
    TextProcessing,
    Process,
    Navigation,
}

impl CheatsheetCategory {
    fn for_tool(tool: &ToolType) -> Vec<CheatsheetCategory> {
        match tool {
            ToolType::Git => vec![
                CheatsheetCategory::All,
                CheatsheetCategory::Basic,
                CheatsheetCategory::Branch,
                CheatsheetCategory::Remote,
                CheatsheetCategory::Advanced,
            ],
            ToolType::Docker => vec![
                CheatsheetCategory::All,
                CheatsheetCategory::Container,
                CheatsheetCategory::Image,
                CheatsheetCategory::Network,
                CheatsheetCategory::Volume,
            ],
            ToolType::Kubernetes => vec![
                CheatsheetCategory::All,
                CheatsheetCategory::Pod,
                CheatsheetCategory::Service,
                CheatsheetCategory::Deployment,
                CheatsheetCategory::Config,
            ],
            ToolType::Tmux => vec![
                CheatsheetCategory::All,
                CheatsheetCategory::Session,
                CheatsheetCategory::Window,
                CheatsheetCategory::Pane,
                CheatsheetCategory::Navigation,
            ],
            ToolType::Bash => vec![
                CheatsheetCategory::All,
                CheatsheetCategory::FileOps,
                CheatsheetCategory::TextProcessing,
                CheatsheetCategory::Process,
                CheatsheetCategory::Navigation,
            ],
        }
    }

    fn translation_key(&self) -> &'static str {
        match self {
            CheatsheetCategory::All => "cheatsheet_viewer.cat_all",
            CheatsheetCategory::Basic => "cheatsheet_viewer.cat_basic",
            CheatsheetCategory::Branch => "cheatsheet_viewer.cat_branch",
            CheatsheetCategory::Remote => "cheatsheet_viewer.cat_remote",
            CheatsheetCategory::Advanced => "cheatsheet_viewer.cat_advanced",
            CheatsheetCategory::Container => "cheatsheet_viewer.cat_container",
            CheatsheetCategory::Image => "cheatsheet_viewer.cat_image",
            CheatsheetCategory::Network => "cheatsheet_viewer.cat_network",
            CheatsheetCategory::Volume => "cheatsheet_viewer.cat_volume",
            CheatsheetCategory::Pod => "cheatsheet_viewer.cat_pod",
            CheatsheetCategory::Service => "cheatsheet_viewer.cat_service",
            CheatsheetCategory::Deployment => "cheatsheet_viewer.cat_deployment",
            CheatsheetCategory::Config => "cheatsheet_viewer.cat_config",
            CheatsheetCategory::Session => "cheatsheet_viewer.cat_session",
            CheatsheetCategory::Window => "cheatsheet_viewer.cat_window",
            CheatsheetCategory::Pane => "cheatsheet_viewer.cat_pane",
            CheatsheetCategory::FileOps => "cheatsheet_viewer.cat_file_ops",
            CheatsheetCategory::TextProcessing => "cheatsheet_viewer.cat_text_processing",
            CheatsheetCategory::Process => "cheatsheet_viewer.cat_process",
            CheatsheetCategory::Navigation => "cheatsheet_viewer.cat_navigation",
        }
    }
}

#[derive(Clone)]
struct CheatsheetEntry {
    command: &'static str,
    desc_en: &'static str,
    desc_ja: &'static str,
    category: CheatsheetCategory,
}

fn get_git_cheatsheet() -> Vec<CheatsheetEntry> {
    vec![
        // Basic
        CheatsheetEntry {
            command: "git init",
            desc_en: "Initialize a new repository",
            desc_ja: "新しいリポジトリを初期化",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git clone <url>",
            desc_en: "Clone a repository",
            desc_ja: "リポジトリをクローン",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git status",
            desc_en: "Show working tree status",
            desc_ja: "作業ツリーの状態を表示",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git add <file>",
            desc_en: "Stage file changes",
            desc_ja: "ファイルの変更をステージ",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git add .",
            desc_en: "Stage all changes",
            desc_ja: "すべての変更をステージ",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git commit -m \"message\"",
            desc_en: "Commit staged changes",
            desc_ja: "ステージした変更をコミット",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git commit --amend",
            desc_en: "Amend last commit",
            desc_ja: "最後のコミットを修正",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git log",
            desc_en: "Show commit history",
            desc_ja: "コミット履歴を表示",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git log --oneline",
            desc_en: "Show compact commit log",
            desc_ja: "コンパクトなコミットログを表示",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git diff",
            desc_en: "Show unstaged changes",
            desc_ja: "ステージされていない変更を表示",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git diff --staged",
            desc_en: "Show staged changes",
            desc_ja: "ステージされた変更を表示",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git rm <file>",
            desc_en: "Remove file from tracking",
            desc_ja: "ファイルを追跡から削除",
            category: CheatsheetCategory::Basic,
        },
        CheatsheetEntry {
            command: "git mv <old> <new>",
            desc_en: "Move or rename a file",
            desc_ja: "ファイルの移動・名前変更",
            category: CheatsheetCategory::Basic,
        },
        // Branch
        CheatsheetEntry {
            command: "git branch",
            desc_en: "List branches",
            desc_ja: "ブランチ一覧を表示",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git branch <name>",
            desc_en: "Create a new branch",
            desc_ja: "新しいブランチを作成",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git checkout <branch>",
            desc_en: "Switch to a branch",
            desc_ja: "ブランチに切り替え",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git checkout -b <branch>",
            desc_en: "Create and switch to a branch",
            desc_ja: "ブランチを作成して切り替え",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git switch <branch>",
            desc_en: "Switch to a branch (new syntax)",
            desc_ja: "ブランチに切り替え（新構文）",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git merge <branch>",
            desc_en: "Merge a branch into current",
            desc_ja: "ブランチを現在のブランチにマージ",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git rebase <branch>",
            desc_en: "Rebase current branch",
            desc_ja: "現在のブランチをリベース",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git branch -d <branch>",
            desc_en: "Delete a branch",
            desc_ja: "ブランチを削除",
            category: CheatsheetCategory::Branch,
        },
        CheatsheetEntry {
            command: "git branch -m <old> <new>",
            desc_en: "Rename a branch",
            desc_ja: "ブランチの名前を変更",
            category: CheatsheetCategory::Branch,
        },
        // Remote
        CheatsheetEntry {
            command: "git remote -v",
            desc_en: "List remote repositories",
            desc_ja: "リモートリポジトリ一覧を表示",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git remote add <name> <url>",
            desc_en: "Add a remote repository",
            desc_ja: "リモートリポジトリを追加",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git fetch",
            desc_en: "Fetch from remote",
            desc_ja: "リモートからフェッチ",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git pull",
            desc_en: "Pull changes from remote",
            desc_ja: "リモートから変更をプル",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git push",
            desc_en: "Push changes to remote",
            desc_ja: "変更をリモートにプッシュ",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git push -u origin <branch>",
            desc_en: "Push and set upstream branch",
            desc_ja: "上流ブランチを設定してプッシュ",
            category: CheatsheetCategory::Remote,
        },
        CheatsheetEntry {
            command: "git push origin --delete <branch>",
            desc_en: "Delete a remote branch",
            desc_ja: "リモートブランチを削除",
            category: CheatsheetCategory::Remote,
        },
        // Advanced
        CheatsheetEntry {
            command: "git stash",
            desc_en: "Stash current changes",
            desc_ja: "現在の変更をスタッシュ",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git stash pop",
            desc_en: "Apply and remove last stash",
            desc_ja: "最後のスタッシュを適用して削除",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git stash list",
            desc_en: "List all stashes",
            desc_ja: "すべてのスタッシュを一覧表示",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git reset HEAD <file>",
            desc_en: "Unstage a file",
            desc_ja: "ファイルのステージを取り消し",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git reset --hard HEAD~1",
            desc_en: "Undo last commit (discard changes)",
            desc_ja: "最後のコミットを取り消し（変更を破棄）",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git reset --soft HEAD~1",
            desc_en: "Undo last commit (keep changes)",
            desc_ja: "最後のコミットを取り消し（変更を保持）",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git cherry-pick <commit>",
            desc_en: "Apply a specific commit",
            desc_ja: "特定のコミットを適用",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git tag <name>",
            desc_en: "Create a tag",
            desc_ja: "タグを作成",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git bisect start",
            desc_en: "Start binary search for a bug",
            desc_ja: "バグの二分探索を開始",
            category: CheatsheetCategory::Advanced,
        },
        CheatsheetEntry {
            command: "git reflog",
            desc_en: "Show reference log",
            desc_ja: "参照ログを表示",
            category: CheatsheetCategory::Advanced,
        },
    ]
}

fn get_docker_cheatsheet() -> Vec<CheatsheetEntry> {
    vec![
        // Container
        CheatsheetEntry {
            command: "docker run <image>",
            desc_en: "Run a container",
            desc_ja: "コンテナを実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker run -d <image>",
            desc_en: "Run container in background",
            desc_ja: "コンテナをバックグラウンドで実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker run -it <image> /bin/bash",
            desc_en: "Run container interactively",
            desc_ja: "コンテナをインタラクティブに実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker run -p 8080:80 <image>",
            desc_en: "Run with port mapping",
            desc_ja: "ポートマッピングで実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker run -v /host:/container <image>",
            desc_en: "Run with volume mount",
            desc_ja: "ボリュームマウントで実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker ps",
            desc_en: "List running containers",
            desc_ja: "実行中のコンテナを一覧表示",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker ps -a",
            desc_en: "List all containers",
            desc_ja: "すべてのコンテナを一覧表示",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker stop <container>",
            desc_en: "Stop a container",
            desc_ja: "コンテナを停止",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker start <container>",
            desc_en: "Start a stopped container",
            desc_ja: "停止したコンテナを開始",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker rm <container>",
            desc_en: "Remove a container",
            desc_ja: "コンテナを削除",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker exec -it <container> /bin/bash",
            desc_en: "Execute command in container",
            desc_ja: "コンテナ内でコマンドを実行",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker logs <container>",
            desc_en: "View container logs",
            desc_ja: "コンテナのログを表示",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker logs -f <container>",
            desc_en: "Follow container logs",
            desc_ja: "コンテナのログをフォロー",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker inspect <container>",
            desc_en: "Inspect container details",
            desc_ja: "コンテナの詳細を確認",
            category: CheatsheetCategory::Container,
        },
        // Image
        CheatsheetEntry {
            command: "docker images",
            desc_en: "List images",
            desc_ja: "イメージを一覧表示",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker pull <image>",
            desc_en: "Pull an image",
            desc_ja: "イメージをプル",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker build -t <name> .",
            desc_en: "Build an image from Dockerfile",
            desc_ja: "Dockerfileからイメージをビルド",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker rmi <image>",
            desc_en: "Remove an image",
            desc_ja: "イメージを削除",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker tag <image> <new_tag>",
            desc_en: "Tag an image",
            desc_ja: "イメージにタグを付ける",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker push <image>",
            desc_en: "Push image to registry",
            desc_ja: "イメージをレジストリにプッシュ",
            category: CheatsheetCategory::Image,
        },
        CheatsheetEntry {
            command: "docker image prune",
            desc_en: "Remove unused images",
            desc_ja: "未使用のイメージを削除",
            category: CheatsheetCategory::Image,
        },
        // Network
        CheatsheetEntry {
            command: "docker network ls",
            desc_en: "List networks",
            desc_ja: "ネットワークを一覧表示",
            category: CheatsheetCategory::Network,
        },
        CheatsheetEntry {
            command: "docker network create <name>",
            desc_en: "Create a network",
            desc_ja: "ネットワークを作成",
            category: CheatsheetCategory::Network,
        },
        CheatsheetEntry {
            command: "docker network connect <net> <container>",
            desc_en: "Connect container to network",
            desc_ja: "コンテナをネットワークに接続",
            category: CheatsheetCategory::Network,
        },
        CheatsheetEntry {
            command: "docker network inspect <name>",
            desc_en: "Inspect network details",
            desc_ja: "ネットワークの詳細を確認",
            category: CheatsheetCategory::Network,
        },
        // Volume
        CheatsheetEntry {
            command: "docker volume ls",
            desc_en: "List volumes",
            desc_ja: "ボリュームを一覧表示",
            category: CheatsheetCategory::Volume,
        },
        CheatsheetEntry {
            command: "docker volume create <name>",
            desc_en: "Create a volume",
            desc_ja: "ボリュームを作成",
            category: CheatsheetCategory::Volume,
        },
        CheatsheetEntry {
            command: "docker volume rm <name>",
            desc_en: "Remove a volume",
            desc_ja: "ボリュームを削除",
            category: CheatsheetCategory::Volume,
        },
        CheatsheetEntry {
            command: "docker volume prune",
            desc_en: "Remove unused volumes",
            desc_ja: "未使用のボリュームを削除",
            category: CheatsheetCategory::Volume,
        },
        CheatsheetEntry {
            command: "docker compose up",
            desc_en: "Start services with Compose",
            desc_ja: "Composeでサービスを起動",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker compose up -d",
            desc_en: "Start services in background",
            desc_ja: "バックグラウンドでサービスを起動",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker compose down",
            desc_en: "Stop and remove services",
            desc_ja: "サービスを停止して削除",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker compose logs",
            desc_en: "View Compose logs",
            desc_ja: "Composeのログを表示",
            category: CheatsheetCategory::Container,
        },
        CheatsheetEntry {
            command: "docker system prune",
            desc_en: "Remove all unused data",
            desc_ja: "未使用のデータをすべて削除",
            category: CheatsheetCategory::Image,
        },
    ]
}

fn get_kubernetes_cheatsheet() -> Vec<CheatsheetEntry> {
    vec![
        // Pod
        CheatsheetEntry {
            command: "kubectl get pods",
            desc_en: "List all pods",
            desc_ja: "すべてのPodを一覧表示",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl get pods -A",
            desc_en: "List pods in all namespaces",
            desc_ja: "すべての名前空間のPodを表示",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl describe pod <name>",
            desc_en: "Show pod details",
            desc_ja: "Podの詳細を表示",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl logs <pod>",
            desc_en: "View pod logs",
            desc_ja: "Podのログを表示",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl logs -f <pod>",
            desc_en: "Follow pod logs",
            desc_ja: "Podのログをフォロー",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl exec -it <pod> -- /bin/bash",
            desc_en: "Execute command in pod",
            desc_ja: "Pod内でコマンドを実行",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl delete pod <name>",
            desc_en: "Delete a pod",
            desc_ja: "Podを削除",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl top pods",
            desc_en: "Show pod resource usage",
            desc_ja: "Podのリソース使用量を表示",
            category: CheatsheetCategory::Pod,
        },
        CheatsheetEntry {
            command: "kubectl port-forward <pod> 8080:80",
            desc_en: "Forward a local port to pod",
            desc_ja: "ローカルポートをPodに転送",
            category: CheatsheetCategory::Pod,
        },
        // Service
        CheatsheetEntry {
            command: "kubectl get svc",
            desc_en: "List all services",
            desc_ja: "すべてのServiceを一覧表示",
            category: CheatsheetCategory::Service,
        },
        CheatsheetEntry {
            command: "kubectl expose deployment <name> --port=80",
            desc_en: "Expose a deployment as service",
            desc_ja: "DeploymentをServiceとして公開",
            category: CheatsheetCategory::Service,
        },
        CheatsheetEntry {
            command: "kubectl describe svc <name>",
            desc_en: "Show service details",
            desc_ja: "Serviceの詳細を表示",
            category: CheatsheetCategory::Service,
        },
        CheatsheetEntry {
            command: "kubectl get endpoints",
            desc_en: "List endpoints",
            desc_ja: "エンドポイントを一覧表示",
            category: CheatsheetCategory::Service,
        },
        CheatsheetEntry {
            command: "kubectl get ingress",
            desc_en: "List ingress resources",
            desc_ja: "Ingressリソースを一覧表示",
            category: CheatsheetCategory::Service,
        },
        // Deployment
        CheatsheetEntry {
            command: "kubectl get deployments",
            desc_en: "List all deployments",
            desc_ja: "すべてのDeploymentを一覧表示",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl create deployment <name> --image=<image>",
            desc_en: "Create a deployment",
            desc_ja: "Deploymentを作成",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl scale deployment <name> --replicas=3",
            desc_en: "Scale a deployment",
            desc_ja: "Deploymentをスケール",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl rollout status deployment/<name>",
            desc_en: "Check rollout status",
            desc_ja: "ロールアウトの状態を確認",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl rollout undo deployment/<name>",
            desc_en: "Rollback a deployment",
            desc_ja: "Deploymentをロールバック",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl set image deployment/<name> <container>=<image>",
            desc_en: "Update container image",
            desc_ja: "コンテナイメージを更新",
            category: CheatsheetCategory::Deployment,
        },
        CheatsheetEntry {
            command: "kubectl rollout history deployment/<name>",
            desc_en: "View rollout history",
            desc_ja: "ロールアウト履歴を表示",
            category: CheatsheetCategory::Deployment,
        },
        // Config
        CheatsheetEntry {
            command: "kubectl apply -f <file.yaml>",
            desc_en: "Apply a configuration",
            desc_ja: "設定を適用",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl delete -f <file.yaml>",
            desc_en: "Delete from configuration",
            desc_ja: "設定からリソースを削除",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl get configmap",
            desc_en: "List ConfigMaps",
            desc_ja: "ConfigMapを一覧表示",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl get secrets",
            desc_en: "List secrets",
            desc_ja: "Secretを一覧表示",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl get namespaces",
            desc_en: "List namespaces",
            desc_ja: "名前空間を一覧表示",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl config get-contexts",
            desc_en: "List contexts",
            desc_ja: "コンテキストを一覧表示",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl config use-context <name>",
            desc_en: "Switch context",
            desc_ja: "コンテキストを切り替え",
            category: CheatsheetCategory::Config,
        },
        CheatsheetEntry {
            command: "kubectl get nodes",
            desc_en: "List cluster nodes",
            desc_ja: "クラスタのノードを一覧表示",
            category: CheatsheetCategory::Config,
        },
    ]
}

fn get_tmux_cheatsheet() -> Vec<CheatsheetEntry> {
    vec![
        // Session
        CheatsheetEntry {
            command: "tmux new -s <name>",
            desc_en: "Create a new session",
            desc_ja: "新しいセッションを作成",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "tmux ls",
            desc_en: "List sessions",
            desc_ja: "セッション一覧を表示",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "tmux attach -t <name>",
            desc_en: "Attach to a session",
            desc_ja: "セッションにアタッチ",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "tmux kill-session -t <name>",
            desc_en: "Kill a session",
            desc_ja: "セッションを終了",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "Ctrl+B, D",
            desc_en: "Detach from session",
            desc_ja: "セッションからデタッチ",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "Ctrl+B, $",
            desc_en: "Rename session",
            desc_ja: "セッション名を変更",
            category: CheatsheetCategory::Session,
        },
        CheatsheetEntry {
            command: "Ctrl+B, S",
            desc_en: "List sessions (interactive)",
            desc_ja: "セッション一覧（インタラクティブ）",
            category: CheatsheetCategory::Session,
        },
        // Window
        CheatsheetEntry {
            command: "Ctrl+B, C",
            desc_en: "Create a new window",
            desc_ja: "新しいウィンドウを作成",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, N",
            desc_en: "Next window",
            desc_ja: "次のウィンドウ",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, P",
            desc_en: "Previous window",
            desc_ja: "前のウィンドウ",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, <number>",
            desc_en: "Switch to window by number",
            desc_ja: "番号でウィンドウを切り替え",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, ,",
            desc_en: "Rename current window",
            desc_ja: "現在のウィンドウ名を変更",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, &",
            desc_en: "Close current window",
            desc_ja: "現在のウィンドウを閉じる",
            category: CheatsheetCategory::Window,
        },
        CheatsheetEntry {
            command: "Ctrl+B, W",
            desc_en: "List windows (interactive)",
            desc_ja: "ウィンドウ一覧（インタラクティブ）",
            category: CheatsheetCategory::Window,
        },
        // Pane
        CheatsheetEntry {
            command: "Ctrl+B, %",
            desc_en: "Split pane vertically",
            desc_ja: "ペインを垂直に分割",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, \"",
            desc_en: "Split pane horizontally",
            desc_ja: "ペインを水平に分割",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, Arrow",
            desc_en: "Move between panes",
            desc_ja: "ペイン間を移動",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, X",
            desc_en: "Close current pane",
            desc_ja: "現在のペインを閉じる",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, Z",
            desc_en: "Toggle pane zoom",
            desc_ja: "ペインのズームを切り替え",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, {",
            desc_en: "Move pane left",
            desc_ja: "ペインを左に移動",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, }",
            desc_en: "Move pane right",
            desc_ja: "ペインを右に移動",
            category: CheatsheetCategory::Pane,
        },
        CheatsheetEntry {
            command: "Ctrl+B, Space",
            desc_en: "Toggle pane layouts",
            desc_ja: "ペインレイアウトを切り替え",
            category: CheatsheetCategory::Pane,
        },
        // Navigation
        CheatsheetEntry {
            command: "Ctrl+B, [",
            desc_en: "Enter copy mode",
            desc_ja: "コピーモードに入る",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "Ctrl+B, ]",
            desc_en: "Paste buffer",
            desc_ja: "バッファを貼り付け",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "Ctrl+B, :",
            desc_en: "Enter command mode",
            desc_ja: "コマンドモードに入る",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "Ctrl+B, ?",
            desc_en: "List key bindings",
            desc_ja: "キーバインド一覧を表示",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "Ctrl+B, T",
            desc_en: "Show time",
            desc_ja: "時刻を表示",
            category: CheatsheetCategory::Navigation,
        },
    ]
}

fn get_bash_cheatsheet() -> Vec<CheatsheetEntry> {
    vec![
        // File Operations
        CheatsheetEntry {
            command: "ls -la",
            desc_en: "List all files with details",
            desc_ja: "詳細付きですべてのファイルを表示",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "cd <dir>",
            desc_en: "Change directory",
            desc_ja: "ディレクトリを変更",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "pwd",
            desc_en: "Print working directory",
            desc_ja: "現在のディレクトリを表示",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "mkdir -p <dir>",
            desc_en: "Create directories recursively",
            desc_ja: "ディレクトリを再帰的に作成",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "cp -r <src> <dest>",
            desc_en: "Copy files/directories recursively",
            desc_ja: "ファイル/ディレクトリを再帰的にコピー",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "mv <src> <dest>",
            desc_en: "Move or rename files",
            desc_ja: "ファイルの移動・名前変更",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "rm -rf <path>",
            desc_en: "Remove files/directories forcefully",
            desc_ja: "ファイル/ディレクトリを強制削除",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "touch <file>",
            desc_en: "Create empty file or update timestamp",
            desc_ja: "空ファイルを作成またはタイムスタンプを更新",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "chmod 755 <file>",
            desc_en: "Change file permissions",
            desc_ja: "ファイルのパーミッションを変更",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "chown user:group <file>",
            desc_en: "Change file owner",
            desc_ja: "ファイルの所有者を変更",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "find . -name \"*.txt\"",
            desc_en: "Find files by name",
            desc_ja: "名前でファイルを検索",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "ln -s <target> <link>",
            desc_en: "Create symbolic link",
            desc_ja: "シンボリックリンクを作成",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "tar -czf archive.tar.gz <dir>",
            desc_en: "Create compressed archive",
            desc_ja: "圧縮アーカイブを作成",
            category: CheatsheetCategory::FileOps,
        },
        CheatsheetEntry {
            command: "tar -xzf archive.tar.gz",
            desc_en: "Extract compressed archive",
            desc_ja: "圧縮アーカイブを展開",
            category: CheatsheetCategory::FileOps,
        },
        // Text Processing
        CheatsheetEntry {
            command: "cat <file>",
            desc_en: "Display file contents",
            desc_ja: "ファイル内容を表示",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "head -n 20 <file>",
            desc_en: "Show first N lines",
            desc_ja: "先頭N行を表示",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "tail -n 20 <file>",
            desc_en: "Show last N lines",
            desc_ja: "末尾N行を表示",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "tail -f <file>",
            desc_en: "Follow file in real-time",
            desc_ja: "ファイルをリアルタイムで追跡",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "grep -r \"pattern\" <dir>",
            desc_en: "Search text recursively",
            desc_ja: "テキストを再帰的に検索",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "sed 's/old/new/g' <file>",
            desc_en: "Replace text in file",
            desc_ja: "ファイル内のテキストを置換",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "awk '{print $1}' <file>",
            desc_en: "Extract columns from file",
            desc_ja: "ファイルからカラムを抽出",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "sort <file>",
            desc_en: "Sort file contents",
            desc_ja: "ファイル内容をソート",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "uniq",
            desc_en: "Remove duplicate lines",
            desc_ja: "重複行を削除",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "wc -l <file>",
            desc_en: "Count lines in file",
            desc_ja: "ファイルの行数をカウント",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "diff <file1> <file2>",
            desc_en: "Compare two files",
            desc_ja: "2つのファイルを比較",
            category: CheatsheetCategory::TextProcessing,
        },
        CheatsheetEntry {
            command: "cut -d',' -f1 <file>",
            desc_en: "Cut columns by delimiter",
            desc_ja: "区切り文字でカラムを切り出し",
            category: CheatsheetCategory::TextProcessing,
        },
        // Process
        CheatsheetEntry {
            command: "ps aux",
            desc_en: "List all processes",
            desc_ja: "すべてのプロセスを一覧表示",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "top",
            desc_en: "Monitor system processes",
            desc_ja: "システムプロセスを監視",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "kill <pid>",
            desc_en: "Terminate a process",
            desc_ja: "プロセスを終了",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "kill -9 <pid>",
            desc_en: "Force kill a process",
            desc_ja: "プロセスを強制終了",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "bg",
            desc_en: "Resume job in background",
            desc_ja: "ジョブをバックグラウンドで再開",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "fg",
            desc_en: "Bring job to foreground",
            desc_ja: "ジョブをフォアグラウンドに移行",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "jobs",
            desc_en: "List background jobs",
            desc_ja: "バックグラウンドジョブを一覧表示",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "nohup <command> &",
            desc_en: "Run command immune to hangup",
            desc_ja: "ハングアップに影響されないコマンドを実行",
            category: CheatsheetCategory::Process,
        },
        CheatsheetEntry {
            command: "lsof -i :<port>",
            desc_en: "Find process using a port",
            desc_ja: "ポートを使用しているプロセスを検索",
            category: CheatsheetCategory::Process,
        },
        // Navigation
        CheatsheetEntry {
            command: "cd ~",
            desc_en: "Go to home directory",
            desc_ja: "ホームディレクトリに移動",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "cd -",
            desc_en: "Go to previous directory",
            desc_ja: "前のディレクトリに移動",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "pushd <dir>",
            desc_en: "Push directory to stack",
            desc_ja: "ディレクトリをスタックにプッシュ",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "popd",
            desc_en: "Pop directory from stack",
            desc_ja: "スタックからディレクトリをポップ",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "which <command>",
            desc_en: "Show command location",
            desc_ja: "コマンドの場所を表示",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "alias ll='ls -la'",
            desc_en: "Create a command alias",
            desc_ja: "コマンドのエイリアスを作成",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "history",
            desc_en: "Show command history",
            desc_ja: "コマンド履歴を表示",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "!!",
            desc_en: "Repeat last command",
            desc_ja: "最後のコマンドを繰り返す",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "export VAR=value",
            desc_en: "Set environment variable",
            desc_ja: "環境変数を設定",
            category: CheatsheetCategory::Navigation,
        },
        CheatsheetEntry {
            command: "echo $VAR",
            desc_en: "Print environment variable",
            desc_ja: "環境変数を表示",
            category: CheatsheetCategory::Navigation,
        },
    ]
}

fn get_cheatsheet(tool: &ToolType) -> Vec<CheatsheetEntry> {
    match tool {
        ToolType::Git => get_git_cheatsheet(),
        ToolType::Docker => get_docker_cheatsheet(),
        ToolType::Kubernetes => get_kubernetes_cheatsheet(),
        ToolType::Tmux => get_tmux_cheatsheet(),
        ToolType::Bash => get_bash_cheatsheet(),
    }
}

fn get_desc_for_lang(entry: &CheatsheetEntry, lang: &str) -> String {
    if lang == "ja" {
        entry.desc_ja.to_string()
    } else {
        entry.desc_en.to_string()
    }
}

#[function_component(CheatsheetViewer)]
pub fn cheatsheet_viewer() -> Html {
    let (i18n, _) = use_translation();
    let selected_tool = use_state(|| ToolType::Git);
    let selected_category = use_state(|| CheatsheetCategory::All);
    let search_query = use_state(String::new);
    let copied_index = use_state(|| Option::<usize>::None);
    let favorites = use_state(Vec::<String>::new);

    let current_lang = if i18n.t("common.copy") == "コピー" {
        "ja"
    } else {
        "en"
    };

    let on_tool_change = {
        let selected_tool = selected_tool.clone();
        let selected_category = selected_category.clone();
        Callback::from(move |tool: ToolType| {
            selected_tool.set(tool);
            selected_category.set(CheatsheetCategory::All);
        })
    };

    let on_category_change = {
        let selected_category = selected_category.clone();
        Callback::from(move |cat: CheatsheetCategory| {
            selected_category.set(cat);
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_clear_search = {
        let search_query = search_query.clone();
        Callback::from(move |_| {
            search_query.set(String::new());
        })
    };

    let entries = get_cheatsheet(&selected_tool);
    let categories = CheatsheetCategory::for_tool(&selected_tool);

    let filtered: Vec<(usize, &CheatsheetEntry)> = entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            if *selected_category != CheatsheetCategory::All && entry.category != *selected_category
            {
                return false;
            }
            if !search_query.is_empty() {
                let query = search_query.to_lowercase();
                let desc = get_desc_for_lang(entry, current_lang).to_lowercase();
                let desc_en = entry.desc_en.to_lowercase();
                let cmd = entry.command.to_lowercase();
                return desc.contains(&query) || desc_en.contains(&query) || cmd.contains(&query);
            }
            true
        })
        .collect();

    let match_count = filtered.len();

    html! {
        <div class="shortcut-dictionary">
            <div class="section shortcut-app-section">
                <h3>{i18n.t("cheatsheet_viewer.select_tool")}</h3>
                <div class="shortcut-app-grid">
                    { for ToolType::all().iter().map(|tool| {
                        let is_active = *selected_tool == *tool;
                        let on_click = {
                            let on_tool_change = on_tool_change.clone();
                            let tool = *tool;
                            Callback::from(move |_| on_tool_change.emit(tool))
                        };
                        let label = i18n.t(tool.translation_key());
                        html! {
                            <button
                                class={classes!("shortcut-app-btn", is_active.then_some("active"))}
                                onclick={on_click}
                            >
                                <span class="shortcut-app-icon">{tool.icon()}</span>
                                <span class="shortcut-app-label">{label}</span>
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="section shortcut-filters-section">
                <div class="shortcut-filters-row">
                    <div class="shortcut-search-wrapper">
                        <input
                            type="text"
                            class="form-input shortcut-search-input"
                            placeholder={i18n.t("cheatsheet_viewer.search_placeholder")}
                            value={(*search_query).clone()}
                            oninput={on_search_change}
                        />
                        if !search_query.is_empty() {
                            <button class="shortcut-search-clear" onclick={on_clear_search}>
                                {"\u{2715}"}
                            </button>
                        }
                    </div>
                </div>

                <div class="shortcut-category-tabs">
                    { for categories.iter().map(|cat| {
                        let is_active = *selected_category == *cat;
                        let on_click = {
                            let on_category_change = on_category_change.clone();
                            let cat = *cat;
                            Callback::from(move |_| on_category_change.emit(cat))
                        };
                        let label = i18n.t(cat.translation_key());
                        html! {
                            <button
                                class={classes!("shortcut-category-btn", is_active.then_some("active"))}
                                onclick={on_click}
                            >
                                {label}
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="section shortcut-results-section">
                <div class="shortcut-results-header">
                    <span class="shortcut-results-count">
                        {i18n.t("cheatsheet_viewer.results_count").replace("{count}", &match_count.to_string())}
                    </span>
                </div>
                <div class="shortcut-table-wrapper">
                    <table class="shortcut-table">
                        <thead>
                            <tr>
                                <th class="shortcut-th-key">{i18n.t("cheatsheet_viewer.col_command")}</th>
                                <th class="shortcut-th-action">{i18n.t("cheatsheet_viewer.col_description")}</th>
                                <th class="shortcut-th-category">{i18n.t("cheatsheet_viewer.col_category")}</th>
                                <th class="shortcut-th-copy"></th>
                            </tr>
                        </thead>
                        <tbody>
                            { for filtered.iter().map(|(idx, entry)| {
                                let desc = get_desc_for_lang(entry, current_lang);
                                let cmd = entry.command;
                                let cat_label = i18n.t(entry.category.translation_key());
                                let is_copied = *copied_index == Some(*idx);
                                let is_fav = favorites.contains(&cmd.to_string());
                                let on_copy = {
                                    let copied_index = copied_index.clone();
                                    let cmd_str = cmd.to_string();
                                    let idx = *idx;
                                    Callback::from(move |_| {
                                        let cmd_str = cmd_str.clone();
                                        let copied_index = copied_index.clone();
                                        if let Some(win) = window() {
                                            let clipboard = win.navigator().clipboard();
                                            let copied_index_clone = copied_index.clone();
                                            spawn_local(async move {
                                                let _ = wasm_bindgen_futures::JsFuture::from(
                                                    clipboard.write_text(&cmd_str),
                                                ).await;
                                                copied_index_clone.set(Some(idx));
                                                let copied_reset = copied_index.clone();
                                                gloo_timers::callback::Timeout::new(2000, move || {
                                                    copied_reset.set(None);
                                                }).forget();
                                            });
                                        }
                                    })
                                };
                                let on_fav = {
                                    let favorites = favorites.clone();
                                    let cmd_str = cmd.to_string();
                                    Callback::from(move |_| {
                                        let mut fav_list = (*favorites).clone();
                                        if fav_list.contains(&cmd_str) {
                                            fav_list.retain(|c| c != &cmd_str);
                                        } else {
                                            fav_list.push(cmd_str.clone());
                                        }
                                        favorites.set(fav_list);
                                    })
                                };
                                html! {
                                    <tr class="shortcut-row">
                                        <td class="shortcut-td-key">
                                            <kbd class="shortcut-kbd">{cmd}</kbd>
                                        </td>
                                        <td class="shortcut-td-action">{desc}</td>
                                        <td class="shortcut-td-category">
                                            <span class="shortcut-cat-badge">{cat_label}</span>
                                        </td>
                                        <td class="shortcut-td-copy">
                                            <button
                                                class={classes!("copy-btn", "shortcut-copy-btn", is_fav.then_some("favorited"))}
                                                onclick={on_fav}
                                                title={i18n.t("cheatsheet_viewer.favorite")}
                                            >
                                                if is_fav {
                                                    {"\u{2605}"}
                                                } else {
                                                    {"\u{2606}"}
                                                }
                                            </button>
                                            <button
                                                class={classes!("copy-btn", "shortcut-copy-btn", is_copied.then_some("copied"))}
                                                onclick={on_copy}
                                            >
                                                if is_copied {
                                                    {"\u{2713}"}
                                                } else {
                                                    {"\u{1f4cb}"}
                                                }
                                            </button>
                                        </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                    if filtered.is_empty() {
                        <div class="shortcut-no-results">
                            {i18n.t("cheatsheet_viewer.no_results")}
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
