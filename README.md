# **DaisyScraper**

## **More About Program:**
This Program is designed for downloads words from WebPages HTML `<body>` tag. The CLI is for overall purposes like downloads data for analysis, the GUI has been designed for aggregate words as a words to teach but also it can be usefull for download words and other aims but this isn't created for them.
Program is working using multiple threads to download data from webpages so the thousands of words will be saving extreamelly fast thanks to Rust programming language and tokio launch asynchronous environment.
The all downloaded words are saved in **files** folder in **.json files** under the names which are their downloaded UNIX Epoch Date and flags using to help GUI in read destination from where words have been downloaded

## **Availeble Work Modes:**
1. **CLI** - using her commands you can download words from multiple webpages or single in one time,
2. **GUI** - using him you can download words from multiple webpages or single in one time and read downloaded words

## **CLI Commands And Commands Usage Examples:**
### **Commands:**
```
    --url <URL/s>... - use this flag for download words from webpage/s
    -u <URL/s>... - shorther implementation of --url flag
```
### **Commands Usage Examples:**
#### **Using Rust Cargo Packages mannager:**
```cargo run -- --url https://github.com/kotekpsotek/DaisyScraper https://github.com/kotekpsotek``` - after that words will be save in **files** folder
<br>
```cargo run -- - https://github.com/kotekpsotek/DaisyScraper https://github.com/kotekpsotek``` - shorther usage of --url flag with the same result

#### **Using compiled file to executable format like .exe or .elf**
```program.exe --url https://github.com/kotekpsotek/DaisyScraper https://github.com/kotekpsotek``` - after that words will be save in **files** folder
<br>
```program.exe -- - https://github.com/kotekpsotek/DaisyScraper https://github.com/kotekpsotek``` - shorther usage of --url flag with the same result

## **Introduction FLTK GUI:**
### **Main GUI Panel:**
**Description:** By use this panel you can: add link/links from where you would like scrap words by enter link in input field and click on "Add Link" button, Download words by click on "Scrap Words" button, go to panel where are all donwloaded words lists by click on "Menu" button located above of bar with input element
<br>

**Overview:**
<br>
<img src="./attachments/Image%20#1.png"/>

### **Sets with lists which contains downladed words:**
**Description:** By use this panel you can: See all lists with words and location from where this words have been downloaded, See how many words are in the particular lists, Get all lists which words has been downloaded from particular URL address by put this URL in input element and then click on "Search" button placed next to, Go to specific words list by click on list to which you would like to go
<br>

**Overview:**
<br>
<img src="./attachments/Image%20#2.png"/>

### **Panel with words from specific list:**
**Description:** By use this panel you can: See all words from list, Select sepcific word/s as a "learned"
<br>

**Overview:**
<br>
<img src="./attachments/Image%20#3.png"/>

## **Used Technologies To Finalize This Project:**
1. **Rust** - Fast, Safe and Productive programming language,
2. **Tokio** - Multi thread asynchronous launch environment for Rust programming language,
3. **FLTK** - Fast and Simple Graphic library created in C++ which offers bindings for Rust programming language,
4. **CLAP** - Outside crate for Rust programming language which simplifies process of creating CLI Applications,
5. **Reqwest** - Outside crate for Rust programming language which allow to make http request in simple way,
6. **Others...** - which merit aren't smaller then above examples

## **License:**
Project is distrybuted under the terms of MIT license