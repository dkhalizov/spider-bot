INSERT INTO tarantula_species (id, scientific_name, common_name, adult_size_cm, temperament, humidity_requirement_percent, temperature_requirement_celsius)
VALUES
    -- Beginner-Friendly Species
    (2, 'Grammostola pulchra', 'Brazilian Black', 15.0, 'Docile', 65, 24.0),
    (3, 'Aphonopelma chalcodes', 'Arizona Blonde', 11.0, 'Docile', 60, 25.0),
    (4, 'Tliltocatl albopilosus', 'Curly Hair', 13.0, 'Docile', 65, 24.0),
    (5, 'Chromatopelma cyaneopubescens', 'Green Bottle Blue', 12.0, 'Moderate', 65, 25.0),
    (6, 'Grammostola pulchripes', 'Chaco Golden Knee', 16.0, 'Docile', 65, 24.0),
    (7, 'Caribena versicolor', 'Martinique Pink Toe', 10.0, 'Docile', 75, 25.0),
    (8, 'Grammostola rosea', 'Chilean Rose', 12.0, 'Docile', 60, 23.0),
    (9, 'Acanthoscurria geniculata', 'Brazilian White Knee', 16.0, 'Moderate', 70, 25.0),
    (10, 'Tliltocatl vagans', 'Mexican Red Rump', 12.0, 'Docile', 65, 24.0),

    -- Additional Common Species
    (11, 'Aphonopelma seemanni', 'Costa Rican Zebra', 14.0, 'Moderate', 70, 25.0),
    (12, 'Brachypelma emilia', 'Mexican Red Leg', 13.0, 'Docile', 65, 24.0),
    (13, 'Nhandu chromatus', 'Brazilian Red and White', 15.0, 'Moderate', 70, 25.0),
    (14, 'Psalmopoeus irminia', 'Venezuelan Suntiger', 12.0, 'Skittish', 75, 25.0),
    (15, 'Lasiodora parahybana', 'Brazilian Salmon Pink', 20.0, 'Moderate', 70, 25.0),
    (16, 'Eupalaestrus campestratus', 'Pink Zebra Beauty', 13.0, 'Docile', 65, 24.0),
    (17, 'Grammostola iheringi', 'Entre Rios', 18.0, 'Docile', 65, 24.0),
    (18, 'Homoeomma chilensis', 'Chilean Flame', 12.0, 'Docile', 60, 23.0),
    (19, 'Thrixopelma cyaneolum', 'Peruvian Blue', 12.0, 'Docile', 65, 24.0),
    (20, 'Tliltocatl verdezi', 'Mexican Rose Grey', 13.0, 'Docile', 65, 24.0),

    -- Intermediate Species
    (21, 'Poecilotheria regalis', 'Indian Ornamental', 16.0, 'Defensive', 75, 26.0),
    (22, 'Pterinochilus murinus', 'Orange Baboon', 13.0, 'Aggressive', 70, 26.0),
    (23, 'Ceratogyrus marshalli', 'Straight Horned Baboon', 14.0, 'Defensive', 70, 26.0),
    (24, 'Heteroscodra maculata', 'Togo Starburst', 12.0, 'Defensive', 80, 26.0),
    (25, 'Cyriopagopus lividus', 'Cobalt Blue', 14.0, 'Defensive', 75, 26.0),

    -- Additional Species
    (26, 'Avicularia avicularia', 'Common Pink Toe', 11.0, 'Docile', 75, 25.0),
    (27, 'Brachypelma boehmei', 'Mexican Fireleg', 14.0, 'Docile', 65, 24.0),
    (28, 'Davus pentaloris', 'Guatemalan Tiger Rump', 10.0, 'Docile', 70, 25.0),
    (29, 'Grammostola actaeon', 'Brazilian Red Rump', 15.0, 'Docile', 65, 24.0),
    (30, 'Harpactira pulchripes', 'Golden Blue Leg Baboon', 12.0, 'Defensive', 70, 25.0),

    -- More Diverse Species
    (31, 'Monocentropus balfouri', 'Socotra Island Blue', 12.0, 'Moderate', 70, 26.0),
    (32, 'Pamphobeteus sp. machala', 'Purple Bloom', 18.0, 'Moderate', 75, 25.0),
    (33, 'Phormictopus sp. purple', 'Purple Giant', 17.0, 'Moderate', 75, 26.0),
    (34, 'Xenesthis immanis', 'Colombian Purple Bloom', 18.0, 'Moderate', 75, 25.0),
    (35, 'Ybyrapora diversipes', 'Amazon Sapphire', 10.0, 'Docile', 80, 26.0),

    -- Additional Beginner-Friendly Species
    (36, 'Neoholothele incei', 'Trinidad Olive', 8.0, 'Docile', 70, 25.0),
    (37, 'Aphonopelma hentzi', 'Texas Brown', 11.0, 'Docile', 60, 24.0),
    (38, 'Euathlus sp. red', 'Chilean Flame Dwarf', 9.0, 'Docile', 60, 22.0),
    (39, 'Homoeomma sp. blue', 'Peruvian Blue Bloom', 12.0, 'Docile', 65, 24.0),
    (40, 'Thrixopelma ockerti', 'Peruvian Flame', 13.0, 'Docile', 65, 24.0),

    -- Beautiful Display Species
    (41, 'Haploclastus devamatha', 'Indian Violet', 16.0, 'Defensive', 80, 26.0),
    (42, 'Chilobrachys fimbriatus', 'Indian Violet Tree Spider', 14.0, 'Defensive', 80, 26.0),
    (43, 'Poecilotheria metallica', 'Gooty Sapphire', 15.0, 'Defensive', 75, 26.0),
    (44, 'Cyriopagopus sp. hati hati', 'Malaysian Earth Tiger', 16.0, 'Defensive', 80, 26.0),
    (45, 'Omothymus violaceopes', 'Singapore Blue', 18.0, 'Defensive', 80, 26.0),

    -- Dwarf Species
    (46, 'Hapalopus sp. Colombia', 'Pumpkin Patch', 7.0, 'Docile', 70, 24.0),
    (47, 'Cyriocosmus elegans', 'Trinidad Dwarf Tiger', 6.0, 'Docile', 70, 25.0),
    (48, 'Neoholothele incei gold', 'Trinidad Olive Gold', 8.0, 'Docile', 70, 25.0),
    (49, 'Homoeomma sp. peru', 'Peru Dwarf', 8.0, 'Docile', 65, 24.0),
    (50, 'Kochiana brunnipes', 'Brazilian Dwarf Beauty', 7.0, 'Docile', 70, 24.0) ON CONFLICT DO NOTHING ;