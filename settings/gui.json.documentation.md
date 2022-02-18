<!-- TODO: This docs -->

# ***This is documentation for keys from gui.json file. Read this guide if you would like change any setting manualy or re-create gui.json file when this file from some reason isn't in his space***

1. **font_file_name** - it is a name of file with fonts which will be use for all application text <u>**(extension of file must be attached to the file)**</u>,
2. **font_color** - it is a color of fonts which will be use in all application **(represented in rgb)**,
3. **main_backround_color_rgb** - it is a background color which will be use in application background,
4. **buttons_1st_backround_color_rgb** - it is a color for buttons which are in two top bars ***(Main Top Bar (Search, Menu buttons) and for Search Bar)*** **<u>(represented in rgb)</u>**
5. **input_backround_color_rgb** - it is a backround color which will be set for **inputs (elements where you can put text) background color** **<u>(represented in rgb)</u>**,
6. **frame_background_color_rgb** - it is a background color for frames being parents of her childrens <u>(parent for childrens in GUI hierarchy)</u> ***(frames are elements where are other elemens that elements e.g: container with added links to scrap)*** <b><u><i>(represented in rgb)</i></u></b>
7. **frame_top_bar_elements_background_color** - it is a background color for elements which parent is frame top bar e.g: **Select All Button** or **Elements Count Info**

<details>
    <summary style="font-size: 18px;"><b>How to set RGB color <u>manualy</u></b></summary>
    <ol>
        <li>Go to page where you can select background from rgb colors range. The good tool for that task is sharing by google below this link: https://www.google.com/search?q=rgb+color+picker</li>
        <li><b>Select 3</b> values separated by "," from RGB label and next <b>Copy</b> that values</li>
        <li>Go to settings/gui.json file and put copied values into "[]" brackets (into one bracket "[]" should be 3 values which is a numbers in range: from <b>0</b> to <b>255</b>) placed in fields which config colors using in application e.g: main_backround_color_rgb</li>
    </ol>
</details>