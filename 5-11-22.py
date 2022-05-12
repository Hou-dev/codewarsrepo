#coding challenge to make a function to take inputs and return input with delimiters removed
#the challenge is to take leave the first word with capitalzie letter if it has it and leave lower case if it comes with it.

text = 'A-B-C'
text1 = 'The-Stealth-Warrior'
text2 = 'the_stealth_warrior'
def to_camel_case(text):
    # this function is use to replace - _ delimiter
    a = text.replace("-", " ").replace("_", " ")
    #split words using space
    a = a.split()
    # join the first word and capitilized every word after
    a = a[0] + ''.join(b.capitalize() for b in a[1:])
    return(a)
    # a =''
    # if '_' in text:
    #     a = text.split('_')
    # elif '-' in text:
    #     a = text.split('-')
    # for a in a:
    #     if a[0][0].islower():
    #         pass
    #     else: 
    #         a += a.title()
    # print(a)
    #a = text.title().replace('-','').replace('_','')
    #print(a)
to_camel_case(text)
to_camel_case(text1)
to_camel_case(text2)
