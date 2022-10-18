// Example/Sources/example/main.swift

import XuexiPackage

let hello = "你好你好"
// count the character
print("------------------- Character counter -------------------")
let generated = count_character_for_given_sentence(hello)
for res in generated {
    print(res.get_character().toString())
}

// load the chinese dictionary
let dictionary = DictionaryWrapper.init()

@available(macOS 10.15, *)
func loadDictionaries() async {
    await dictionary.load_chinese_dictionary(XuexiCNVersion.Traditional)
    await dictionary.load_laotian_dictionary()
}

func getDefFromChineseSentences() {
    let sentence = "晚上好大家好我是法國人我覺得很好"
    let output = dictionary.search_in_dictionaries(XuexiLibLanguage.Chinese, sentence)
    if let res = output {
        print("------------------- Chinese -------------------")
        print(res.toString())
    }

    // csv example
    let csv = dictionary.search_and_export(XuexiLibLanguage.Chinese, sentence)
    if let content = csv {
        print("------------------- CSV -------------------")
        print(content.toString())
    }
}

func getDefFromLaotianSentence() {
    let sentence = "ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ"
    let output = dictionary.search_in_dictionaries(XuexiLibLanguage.Laotian, sentence)
    if let res = output {
        print("------------------- Laotian -------------------")
        print(res.toString())
    }
}

if #available(macOS 10.15, *) {
    await loadDictionaries()
    getDefFromChineseSentences()
    getDefFromLaotianSentence()
} else {
    // Fallback on earlier versions
    print("not supported")
}