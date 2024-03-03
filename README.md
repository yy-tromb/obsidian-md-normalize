# obsidian-md-normalize
Convert obsidian type markdown to normal markdown ***For me***
## Features

### Fix picture embeded  
In obsidian markdown, it is `![[path]]`. This tool replace it `![](path)`  
This is written `<img />` in HTML.  
Target picture file extentions are "jpeg", "jpg", "png", "gif", "bmp", "webp", "tiff", "apng".  
You will be able to add some picture extentions. I don't implement now.  

### Enabled new line  
In obsidian app view, new line is enabled, but normal markdown ignore new line. So add "&nbsp;&nbsp;" (two spaces) before new line char like \n or \r\n. 
