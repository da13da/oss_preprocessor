# oss_preprocessor
1. lockファイルからライブラリを取得し、最新バージョンとのdiffを収集する。
2. 取得したdiffをそれぞれの言語のASTにparseする。
3. parse後前処理した結果をoutputし、機械学習に使用する。
