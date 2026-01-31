---
name: ios-design
description: iOSネイティブアプリ風の洗練されたUIデザインを作成。クリーンで直感的、かつ高品質なインターフェースを実現するための具体的なガイドライン。
---

# iOS Native Design Skill

iOSネイティブアプリのような洗練されたUIを作成するためのガイドライン。Appleのヒューマンインターフェースガイドラインに基づき、クリーンで直感的なデザインを実現する。

## デザイン原則

### 1. Clarity（明瞭性）
- テキストは読みやすいサイズで
- アイコンは正確で理解しやすく
- 装飾は機能に従う

### 2. Deference（控えめさ）
- UIはコンテンツを引き立てる存在
- 流動的なモーションで空間の理解を助ける
- 半透明とブラーで奥行きを表現

### 3. Depth（奥行き）
- レイヤーと動きで階層を表現
- タッチによる即座のフィードバック
- コンテキストに応じた遷移

## 避けるべきパターン（AIっぽいデザイン）

| 避けるべき | 代わりに使う |
|-----------|-------------|
| 紫/インディゴのグラデーション | iOS System Colors |
| Inter, Roboto などの汎用フォント | System font (-apple-system) |
| 12-16pxの中途半端なborder-radius | 8px, 12px, 20px の階層 |
| 浮かび上がるホバーエフェクト | 押し込まれるタッチフィードバック |
| 純白 `#ffffff` の背景 | iOS Gray `#f2f2f7` |
| 純黒 `#000000` のテキスト | System Label `rgba(0,0,0,0.85)` |
| 派手なドロップシャドウ | 控えめな `0 1px 3px rgba(0,0,0,0.1)` |
| ランダムなアクセントカラー | iOS Tint `#007aff` |

## カラーシステム

```css
:root {
  /* ===== iOS System Colors ===== */

  /* Background */
  --color-bg-primary: #f2f2f7;
  --color-bg-secondary: #ffffff;
  --color-bg-tertiary: #e5e5ea;
  --color-bg-grouped: #f2f2f7;

  /* Label */
  --color-label-primary: rgba(0, 0, 0, 0.85);
  --color-label-secondary: rgba(60, 60, 67, 0.6);
  --color-label-tertiary: rgba(60, 60, 67, 0.3);
  --color-label-quaternary: rgba(60, 60, 67, 0.18);

  /* Separator */
  --color-separator: rgba(60, 60, 67, 0.12);
  --color-separator-opaque: #c6c6c8;

  /* System Tints */
  --color-tint-blue: #007aff;
  --color-tint-green: #34c759;
  --color-tint-indigo: #5856d6;
  --color-tint-orange: #ff9500;
  --color-tint-pink: #ff2d55;
  --color-tint-purple: #af52de;
  --color-tint-red: #ff3b30;
  --color-tint-teal: #5ac8fa;
  --color-tint-yellow: #ffcc00;

  /* Fill */
  --color-fill-primary: rgba(120, 120, 128, 0.2);
  --color-fill-secondary: rgba(120, 120, 128, 0.16);
  --color-fill-tertiary: rgba(118, 118, 128, 0.12);
  --color-fill-quaternary: rgba(116, 116, 128, 0.08);
}

/* ===== Dark Mode ===== */
@media (prefers-color-scheme: dark) {
  :root {
    --color-bg-primary: #000000;
    --color-bg-secondary: #1c1c1e;
    --color-bg-tertiary: #2c2c2e;
    --color-bg-grouped: #000000;

    --color-label-primary: rgba(255, 255, 255, 0.85);
    --color-label-secondary: rgba(235, 235, 245, 0.6);
    --color-label-tertiary: rgba(235, 235, 245, 0.3);
    --color-label-quaternary: rgba(235, 235, 245, 0.16);

    --color-separator: rgba(84, 84, 88, 0.6);
    --color-separator-opaque: #38383a;

    --color-fill-primary: rgba(120, 120, 128, 0.36);
    --color-fill-secondary: rgba(120, 120, 128, 0.32);
    --color-fill-tertiary: rgba(118, 118, 128, 0.24);
    --color-fill-quaternary: rgba(116, 116, 128, 0.18);
  }
}
```

## タイポグラフィ

```css
:root {
  /* SF Pro風のシステムフォント */
  --font-system: -apple-system, BlinkMacSystemFont, 'SF Pro Text',
                 'Helvetica Neue', 'Hiragino Sans', 'Hiragino Kaku Gothic ProN',
                 'Noto Sans JP', sans-serif;

  --font-display: -apple-system, BlinkMacSystemFont, 'SF Pro Display',
                  'Helvetica Neue', sans-serif;

  --font-mono: 'SF Mono', ui-monospace, 'Menlo', monospace;

  /* Dynamic Type Scale */
  --text-large-title: 34px;    /* Large Title */
  --text-title1: 28px;         /* Title 1 */
  --text-title2: 22px;         /* Title 2 */
  --text-title3: 20px;         /* Title 3 */
  --text-headline: 17px;       /* Headline (semibold) */
  --text-body: 17px;           /* Body */
  --text-callout: 16px;        /* Callout */
  --text-subhead: 15px;        /* Subhead */
  --text-footnote: 13px;       /* Footnote */
  --text-caption1: 12px;       /* Caption 1 */
  --text-caption2: 11px;       /* Caption 2 */

  /* Line Height */
  --leading-tight: 1.2;
  --leading-normal: 1.4;
  --leading-relaxed: 1.6;

  /* Font Weight */
  --weight-regular: 400;
  --weight-medium: 500;
  --weight-semibold: 600;
  --weight-bold: 700;
}
```

## スペーシング & レイアウト

```css
:root {
  /* iOS標準マージン */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 20px;
  --spacing-xl: 32px;

  /* Safe Area (Tauriでは通常不要) */
  --safe-area-top: env(safe-area-inset-top, 0px);
  --safe-area-bottom: env(safe-area-inset-bottom, 0px);

  /* Border Radius - Continuous Corners */
  --radius-xs: 4px;
  --radius-sm: 8px;     /* Small elements */
  --radius-md: 12px;    /* Cards, buttons */
  --radius-lg: 20px;    /* Large cards, sheets */
  --radius-xl: 38px;    /* Full rounded (Dynamic Island風) */

  /* Shadow */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 2px 8px rgba(0, 0, 0, 0.08);
  --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.12);
}
```

## コンポーネントスタイル

### ボタン

```css
/* Primary Button (Filled) */
.btn-primary {
  background: var(--color-tint-blue);
  color: #ffffff;
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  padding: 14px 20px;
  border: none;
  border-radius: var(--radius-md);
  min-height: 50px;
  transition: opacity 0.15s ease, transform 0.1s ease;
}

.btn-primary:hover {
  opacity: 0.85;
}

.btn-primary:active {
  transform: scale(0.98);
  opacity: 0.7;
}

/* Secondary Button (Gray) */
.btn-secondary {
  background: var(--color-fill-secondary);
  color: var(--color-tint-blue);
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  padding: 14px 20px;
  border: none;
  border-radius: var(--radius-md);
}

/* Text Button */
.btn-text {
  background: transparent;
  color: var(--color-tint-blue);
  font-size: var(--text-body);
  font-weight: var(--weight-regular);
  padding: 8px 12px;
  border: none;
}
```

### カード（Grouped Style）

```css
.card {
  background: var(--color-bg-secondary);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.card-inset {
  margin: 0 var(--spacing-md);
}

/* List Item in Card */
.card-item {
  padding: var(--spacing-md);
  border-bottom: 0.5px solid var(--color-separator);
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.card-item:last-child {
  border-bottom: none;
}

/* Card Header */
.card-header {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--text-footnote);
  color: var(--color-label-secondary);
  text-transform: uppercase;
  letter-spacing: 0.02em;
}
```

### フォーム入力

```css
.input {
  background: var(--color-bg-secondary);
  border: none;
  border-radius: var(--radius-sm);
  padding: 12px 16px;
  font-size: var(--text-body);
  color: var(--color-label-primary);
  width: 100%;
}

.input::placeholder {
  color: var(--color-label-tertiary);
}

.input:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.3);
}

/* Search Bar */
.search-bar {
  background: var(--color-fill-tertiary);
  border-radius: var(--radius-sm);
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-bar input {
  background: transparent;
  border: none;
  flex: 1;
  font-size: var(--text-body);
}
```

### ナビゲーションバー

```css
.navbar {
  background: rgba(249, 249, 249, 0.94);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-bottom: 0.5px solid var(--color-separator);
  padding: var(--spacing-sm) var(--spacing-md);
  position: sticky;
  top: 0;
  z-index: 100;
}

.navbar-title {
  font-size: var(--text-headline);
  font-weight: var(--weight-semibold);
  text-align: center;
}

/* Large Title Style */
.navbar-large-title {
  font-size: var(--text-large-title);
  font-weight: var(--weight-bold);
  padding: var(--spacing-sm) var(--spacing-md);
}
```

## アニメーション

```css
:root {
  /* iOS-style Timing Functions */
  --ease-ios: cubic-bezier(0.25, 0.1, 0.25, 1);
  --ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  --ease-out-expo: cubic-bezier(0.19, 1, 0.22, 1);

  /* Duration */
  --duration-fast: 0.15s;
  --duration-normal: 0.25s;
  --duration-slow: 0.4s;
}

/* Fade In Up */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Scale In */
@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* Slide In From Right */
@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* Usage */
.animate-in {
  animation: fadeInUp var(--duration-normal) var(--ease-ios);
}

.animate-scale {
  animation: scaleIn var(--duration-fast) var(--ease-spring);
}
```

## Yew実装パターン

### ボタンコンポーネント

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(Default, PartialEq, Clone)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Text,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Text => "btn-text",
    };

    html! {
        <button
            class={class}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            {&props.label}
        </button>
    }
}
```

### カードコンポーネント

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub header: Option<String>,
    #[prop_or_default]
    pub inset: bool,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let card_class = if props.inset { "card card-inset" } else { "card" };

    html! {
        <div class={card_class}>
            if let Some(header) = &props.header {
                <div class="card-header">{header}</div>
            }
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardItemProps {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(CardItem)]
pub fn card_item(props: &CardItemProps) -> Html {
    html! {
        <div class="card-item" onclick={props.onclick.clone()}>
            {for props.children.iter()}
        </div>
    }
}
```

## デザインチェックリスト

実装後、以下を確認:

### カラー
- [ ] 背景に `#f2f2f7` (Light) / `#000000` (Dark) を使用
- [ ] テキストに純黒 `#000000` を避け、`rgba(0,0,0,0.85)` を使用
- [ ] アクセントカラーは iOS System Colors から選択
- [ ] 区切り線は `rgba(60, 60, 67, 0.12)` の薄いもの

### タイポグラフィ
- [ ] システムフォント (`-apple-system`) を使用
- [ ] Dynamic Type Scale に準拠したサイズ
- [ ] 適切な font-weight の使い分け

### スペーシング
- [ ] 標準マージン (16px, 20px) の一貫した使用
- [ ] 十分な padding でタッチターゲット確保 (最小44px)

### インタラクション
- [ ] タッチフィードバック（押し込み `scale(0.98)` + opacity変化）
- [ ] 適切なトランジション時間 (0.15s - 0.4s)
- [ ] iOS標準のイージング関数使用

### コンポーネント
- [ ] border-radius は 8px / 12px / 20px の階層
- [ ] シャドウは控えめに
- [ ] Grouped Style のカードレイアウト

### アクセシビリティ
- [ ] 十分なコントラスト比
- [ ] フォーカス状態の視覚的フィードバック
- [ ] 適切な semantic HTML
