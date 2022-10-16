// Example/Sources/example/main.swift

import XuexiPackage

let hello = "你好你好"
// count the character
let generated = count_character_for_given_sentence(hello)
for res in generated {
    print(res.get_character().toString())
    print(res.get_count())
}

// load the chinese dictionary
let dictionary = DictionaryWrapper.init()

@available(macOS 10.15, *)
func loadChineseDictionary() async {
    await dictionary.load_chinese_dictionary(XuexiCNVersion.Traditional)
}

func getDefFromChineseSentences() {
    let sentence = "晚上好大家好我是法國人我覺得很好"
    let output = dictionary.search_in_dictionaries(XuexiLibLanguage.Chinese, sentence)
    if let res = output {
        print(res.toString())
    }
}

if #available(macOS 10.15, *) {
    await loadChineseDictionary()
    getDefFromChineseSentences()
} else {
    // Fallback on earlier versions
    print("not supported")
}