import names
import random

with open("outfile.csv", 'w') as f_out:
    for i in range(5000):
        ci = random.randint(0, 9999999999999)
        random_name=random.randint(0, 2)
        if random_name==0:
            name = names.get_full_name().upper()
        elif random_name==1:
            name=names.get_first_name().upper()
        else:
            name=names.get_last_name().upper()
        gender = random.choice(["M", "F", "NULL",""])
        civil = random.choice(["Soltero", "Casado", "Divorciado","Viudo","Union de hecho","NULL",""])
        nac_day=random.randint(1, 33)
        nac_month=random.randint(1, 14)
        nac_year=random.randint(1, 2022)
        nac = str(nac_year)+"-"+str(nac_month)+"-"+str(nac_day)
        phone = random.randint(0, 99999999999)
        adrs = name
        name_mail=names.get_first_name()
        mail = name_mail+"@gmail.com"
        finalline = str(ci)+";"+name+";"+gender+";"+civil + \
            ";"+nac+";"+str(phone)+";"+adrs+";"+mail+"\n"
        #print(finalline)
        f_out.write(finalline)
