


## 실행 및 옵션
key: usize
mode: encoding | decoding
file_path: xxx.txt

```sh
% cargo run -- 13 encoding frankenstein.txt
```



## 복호화
```sh
% cargo run -- 13 decoding frankenstein.encrypted.txt
```



##  출력결과를 파일로 저장
```sh
$ cargo run -- 13 encoding frankenstein.txt > frankenstein.encrypted.txt
```
