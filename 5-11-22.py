text = 'A-B-C'
text1 = 'The-Stealth-Warrior'
text2 = 'the_stealth_warrior'
def to_camel_case(text):
    a = text.title().replace("-", " ").replace("_", " ")
    a = a.split()
    a = a[0] + ''.join(b.capitalize() for b in a[1:])
    print(a)
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
