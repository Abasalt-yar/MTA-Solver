# MTA-Solver
Solve MTA Issues And Update

# Build
to build, you only have to do 
</br>
`cargo build --release`
</br>
And for development build or debug use
</br>
`cargo run`

# Verify Resources
1. To use this feature, you have to replace the url in 
</br>
`src/modules/verify_resources.rs`
</br>
2. your url must have the resources as follows:
</br>
`│   manifest.xml` </br>
`└───http-client-files` </br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`    └───resourceA`
</br>
3. your `manifest.xml`'s content must be like this: 
</br>
```xml
<base>
    <folder name="resourceA">
        <file name="file.lua" size="sizeInBytes"/>
        <file name="meta.xml" size="sizeInBytes"/>
    </folder>
</base>
```
4. You Are Ready To Go.

# Contact
Please Report Any Bugs Or Requested Features To 
</br>Email: [AbasaltYarmohammadzey@gmail.com](mailto:AbasaltYarmohammadzey@gmail.com)
</br>Telegram: [@Abasalt_Yar](https://t.me/Abasalt_Yar)
</br>Discord: abasalt_yar