DELETE FROM feeding_schedules;

DELETE FROM sqlite_sequence WHERE name='feeding_schedules';

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type, notes)
VALUES
-- Brachypelma hamorii (Mexican Red Knee) - ID: 1
(1, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Very delicate at this stage; ensure food size is no larger than carapace'),
(1, 'Juvenile', 3.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good eater at this stage; watch for premolt signs'),
(1, 'Sub-Adult', 8.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'May fast before molting; ensure proper humidity'),
(1, 'Adult', 14.0, '3-4 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Adjust feeding based on abdomen size; may refuse food during breeding season'),

-- Grammostola pulchra (Brazilian Black) - ID: 2
(2, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow growing species; do not overfeed'),
(2, 'Juvenile', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Consistent feeding important for growth'),
(2, 'Sub-Adult', 9.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'May refuse food more frequently than other species'),
(2, 'Adult', 15.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Known for long fasting periods; dont worry if refusing food'),

-- Aphonopelma chalcodes (Arizona Blonde) - ID: 3
(3, 'Spiderling', 0.4, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Desert species - dont overfeed'),
(3, 'Juvenile', 2.5, '1-2 small crickets', 'Every 7-10 days', 'Small crickets', 'Adapted to infrequent feeding'),
(3, 'Sub-Adult', 6.0, '2 medium crickets', 'Every 14-21 days', 'Medium crickets, small roaches', 'May fast during winter months'),
(3, 'Adult', 11.0, '2-3 medium crickets', 'Every 21-30 days', 'Large crickets, adult roaches', 'Long fasting periods normal; adapted to desert conditions'),

-- Tliltocatl albopilosus (Curly Hair) - ID: 4
(4, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Good eater at all stages'),
(4, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Maintains good appetite through growth'),
(4, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Watch for premolt signs'),
(4, 'Adult', 13.0, '2-3 large crickets', 'Every 14 days', 'Large crickets, adult roaches', 'Reliable eater even as adult'),

-- Chromatopelma cyaneopubescens (Green Bottle Blue) - ID: 5
(5, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Heavy webber - place prey in web'),
(5, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Will catch prey in web'),
(5, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Place prey near web structures'),
(5, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Heavy webber - ensure prey contacts web'),

-- Grammostola pulchripes (Chaco Golden Knee) - ID: 6
(6, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Gentle giant - careful with prey size'),
(6, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good growth rate with regular feeding'),
(6, 'Sub-Adult', 10.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Excellent eater at this stage'),
(6, 'Adult', 16.0, '3-4 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Can take larger prey items due to size'),

-- Caribena versicolor (Martinique Pink Toe) - ID: 7
(7, 'Spiderling', 0.4, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies, pinhead crickets', 'Delicate at this stage; keep well ventilated'),
(7, 'Juvenile', 2.5, '2 small crickets', 'Every 4-5 days', 'Small crickets, flying insects', 'Arboreal - may hunt from cork bark'),
(7, 'Sub-Adult', 5.0, '2-3 medium crickets', 'Every 7 days', 'Medium crickets, moths', 'Prefers aerial prey; will catch in web'),
(7, 'Adult', 10.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, moths, flying insects', 'Place prey on vertical surfaces'),

-- Grammostola rosea (Chilean Rose) - ID: 8
(8, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow growing species'),
(8, 'Juvenile', 3.0, '1-2 small crickets', 'Every 7-10 days', 'Small crickets, small roaches', 'May have irregular feeding patterns'),
(8, 'Sub-Adult', 7.0, '2 medium crickets', 'Every 14-21 days', 'Medium crickets, medium roaches', 'Known for fasting periods'),
(8, 'Adult', 12.0, '2-3 large crickets', 'Every 21-30 days', 'Large crickets, adult roaches', 'Famous for long fasting periods; dont worry if refusing food'),

-- Acanthoscurria geniculata (Brazilian White Knee) - ID: 9
(9, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Aggressive feeder from early age'),
(9, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Fast growing species'),
(9, 'Sub-Adult', 9.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Voracious eater'),
(9, 'Adult', 16.0, '4-5 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Known for large appetite'),

-- Tliltocatl vagans (Mexican Red Rump) - ID: 10
(10, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Hardy species from spiderling'),
(10, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good growth rate with regular feeding'),
(10, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Consistent eater through growth'),
(10, 'Adult', 12.0, '2-3 large crickets', 'Every 14 days', 'Large crickets, adult roaches', 'Maintains good appetite as adult');

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type, notes)
VALUES
-- Aphonopelma seemanni (Costa Rican Zebra) - ID: 11
(11, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Moderate growth rate at this stage'),
(11, 'Juvenile', 3.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'May dig burrows - place prey near entrance'),
(11, 'Sub-Adult', 8.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Watch for burrow expansion before molting'),
(11, 'Adult', 14.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'May seal burrow during premolt; wait until reopened to feed'),

-- Brachypelma emilia (Mexican Red Leg) - ID: 12
(12, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slower growing than other Brachypelma'),
(12, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Steady growth with consistent feeding'),
(12, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Watch for premolt signs'),
(12, 'Adult', 13.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'May fast during breeding season'),

-- Nhandu chromatus (Brazilian Red and White) - ID: 13
(13, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing species from early age'),
(13, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Aggressive feeder during growth'),
(13, 'Sub-Adult', 9.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Maintains strong feeding response'),
(13, 'Adult', 15.0, '3-4 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Can be a defensive feeder - use caution'),

-- Psalmopoeus irminia (Venezuelan Suntiger) - ID: 14
(14, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing arboreal species'),
(14, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Place prey on cork bark or web'),
(14, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Excellent web hunter'),
(14, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, moths, flying insects', 'Prefers aerial prey; may refuse ground prey'),

-- Lasiodora parahybana (Brazilian Salmon Pink) - ID: 15
(15, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing giant species'),
(15, 'Juvenile', 5.0, '3-4 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Requires substantial feeding'),
(15, 'Sub-Adult', 12.0, '4-5 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Maintain heavy feeding schedule'),
(15, 'Adult', 20.0, '5-6 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Largest appetite of common species'),

-- Eupalaestrus campestratus (Pink Zebra Beauty) - ID: 16
(16, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow growing species'),
(16, 'Juvenile', 3.0, '1-2 small crickets', 'Every 7-10 days', 'Small crickets, small roaches', 'Moderate feeding response'),
(16, 'Sub-Adult', 7.0, '2 medium crickets', 'Every 14-21 days', 'Medium crickets, medium roaches', 'May fast during winter months'),
(16, 'Adult', 13.0, '2-3 large crickets', 'Every 21-30 days', 'Large crickets, adult roaches', 'Adapted to infrequent feeding'),

-- Grammostola iheringi (Entre Rios) - ID: 17
(17, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow but steady growth rate'),
(17, 'Juvenile', 4.5, '2-3 small crickets', 'Every 7-10 days', 'Small crickets, small roaches', 'Regular feeding important for growth'),
(17, 'Sub-Adult', 11.0, '3-4 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Good appetite at this stage'),
(17, 'Adult', 18.0, '4-5 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Large species with hearty appetite'),

-- Homoeomma chilensis (Chilean Flame) - ID: 18
(18, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Moderate growth rate'),
(18, 'Juvenile', 3.0, '1-2 small crickets', 'Every 7-10 days', 'Small crickets, small roaches', 'Steady feeder during growth'),
(18, 'Sub-Adult', 7.0, '2 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'May fast more in cooler temperatures'),
(18, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Adapted to temperature fluctuations'),

-- Thrixopelma cyaneolum (Peruvian Blue) - ID: 19
(19, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Delicate at small sizes'),
(19, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good feeding response'),
(19, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Consistent eater through growth'),
(19, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Maintain regular feeding schedule'),

-- Tliltocatl verdezi (Mexican Rose Grey) - ID: 20
(20, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Hardy species from early age'),
(20, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good growth with regular feeding'),
(20, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Watch for premolt signs'),
(20, 'Adult', 13.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Similar care to other Tliltocatl species');

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type, notes)
VALUES
-- Poecilotheria regalis (Indian Ornamental) - ID: 21
(21, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing arboreal; use caution from early age'),
(21, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Place prey on cork bark; use tongs'),
(21, 'Sub-Adult', 9.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Aggressive hunter; excellent web ambusher'),
(21, 'Adult', 16.0, '3-4 medium crickets', 'Every 14-21 days', 'Medium crickets, moths, flying insects', 'Very defensive; use extreme caution when feeding'),

-- Pterinochilus murinus (Orange Baboon) - ID: 22
(22, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing; defensive from early age'),
(22, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Use long tongs; ensure clear retreat path'),
(22, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Extremely defensive; careful during maintenance'),
(22, 'Adult', 13.0, '2-3 medium crickets', 'Every 14 days', 'Medium crickets, adult roaches', 'Feed with extreme caution; best fed at night'),

-- Ceratogyrus marshalli (Straight Horned Baboon) - ID: 23
(23, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Defensive from young age'),
(23, 'Juvenile', 3.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Will rush prey aggressively'),
(23, 'Sub-Adult', 8.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Use caution during feeding'),
(23, 'Adult', 14.0, '2-3 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Keep clear distance when feeding'),

-- Heteroscodra maculata (Togo Starburst) - ID: 24
(24, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fragile arboreal spiderling'),
(24, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Expert web builder; place prey in web'),
(24, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Very fast and defensive'),
(24, 'Adult', 12.0, '2-3 medium crickets', 'Every 14 days', 'Medium crickets, moths, flying insects', 'Defensive arboreal; use long tongs'),

-- Cyriopagopus lividus (Cobalt Blue) - ID: 25
(25, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Provide deep substrate early'),
(25, 'Juvenile', 3.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Will ambush from burrow'),
(25, 'Sub-Adult', 8.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Deep burrower; place prey near entrance'),
(25, 'Adult', 14.0, '2-3 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Highly defensive; feed with long tongs'),

-- Avicularia avicularia (Common Pink Toe) - ID: 26
(26, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Delicate arboreal spiderling'),
(26, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Will hunt from webbed retreat'),
(26, 'Sub-Adult', 6.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Active hunter in evening'),
(26, 'Adult', 11.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, moths, flying insects', 'Place prey on web or vertical surfaces'),

-- Brachypelma boehmei (Mexican Fireleg) - ID: 27
(27, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow growing species'),
(27, 'Juvenile', 3.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Steady growth with regular feeding'),
(27, 'Sub-Adult', 8.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Good feeding response'),
(27, 'Adult', 14.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Similar care to other Brachypelma'),

-- Davus pentaloris (Guatemalan Tiger Rump) - ID: 28
(28, 'Spiderling', 0.4, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Smaller species from spiderling'),
(28, 'Juvenile', 2.5, '1-2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good feeding response'),
(28, 'Sub-Adult', 5.0, '2 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Active hunter'),
(28, 'Adult', 10.0, '2-3 medium crickets', 'Every 14 days', 'Medium crickets, adult roaches', 'Maintains good appetite as adult'),

-- Grammostola actaeon (Brazilian Red Rump) - ID: 29
(29, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Typical Grammostola growth rate'),
(29, 'Juvenile', 3.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Regular feeding promotes growth'),
(29, 'Sub-Adult', 9.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, medium roaches', 'Good appetite through growth'),
(29, 'Adult', 15.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Can be fussy eater as adult'),

-- Harpactira pulchripes (Golden Blue Leg Baboon) - ID: 30
(30, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing baboon species'),
(30, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Use caution when feeding'),
(30, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Fast and defensive'),
(30, 'Adult', 12.0, '2-3 medium crickets', 'Every 14 days', 'Medium crickets, adult roaches', 'Feed with long tongs; ensure retreat path');

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type, notes)
VALUES
-- Monocentropus balfouri (Socotra Island Blue) - ID: 31
(31, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Can be communal; adjust feeding accordingly'),
(31, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good feeding response'),
(31, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Watch for molting in communal setups'),
(31, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'May share prey in communal settings'),

-- Pamphobeteus sp. machala (Purple Bloom) - ID: 32
(32, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing from early age'),
(32, 'Juvenile', 4.5, '3-4 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Heavy feeder during growth'),
(32, 'Sub-Adult', 11.0, '4-5 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Maintains strong feeding response'),
(32, 'Adult', 18.0, '4-5 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Large appetite throughout adulthood'),

-- Phormictopus sp. purple (Purple Giant) - ID: 33
(33, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Aggressive feeder from start'),
(33, 'Juvenile', 4.0, '3-4 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Fast growing with good appetite'),
(33, 'Sub-Adult', 10.0, '4-5 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Voracious eater'),
(33, 'Adult', 17.0, '4-5 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Maintains large appetite as adult'),

-- Xenesthis immanis (Colombian Purple Bloom) - ID: 34
(34, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing species'),
(34, 'Juvenile', 4.5, '3-4 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Strong feeding response'),
(34, 'Sub-Adult', 11.0, '4-5 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Aggressive feeder'),
(34, 'Adult', 18.0, '4-5 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'One of the largest appetites'),

-- Ybyrapora diversipes (Amazon Sapphire) - ID: 35
(35, 'Spiderling', 0.4, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Delicate arboreal spiderling'),
(35, 'Juvenile', 2.5, '2 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Place prey on web or cork bark'),
(35, 'Sub-Adult', 5.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Good web hunter'),
(35, 'Adult', 10.0, '2-3 medium crickets', 'Every 14-21 days', 'Medium crickets, moths, flying insects', 'Prefers aerial prey items'),

-- Neoholothele incei (Trinidad Olive) - ID: 36
(36, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Tiny spiderling; careful not to overfeed'),
(36, 'Juvenile', 1.5, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Heavy webber at all stages'),
(36, 'Sub-Adult', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Place prey in web'),
(36, 'Adult', 8.0, '2-3 medium crickets', 'Every 10-14 days', 'Medium crickets, small roaches', 'Dwarf species; dont overfeed'),

-- Aphonopelma hentzi (Texas Brown) - ID: 37
(37, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Slow growing desert species'),
(37, 'Juvenile', 2.5, '1-2 small crickets', 'Every 7-10 days', 'Small crickets', 'Adapted to infrequent feeding'),
(37, 'Sub-Adult', 6.0, '2 medium crickets', 'Every 14-21 days', 'Medium crickets, small roaches', 'May fast during winter months'),
(37, 'Adult', 11.0, '2-3 medium crickets', 'Every 21-30 days', 'Large crickets, adult roaches', 'Long fasting periods normal'),

-- Euathlus sp. red (Chilean Flame Dwarf) - ID: 38
(38, 'Spiderling', 0.4, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Very small spiderling'),
(38, 'Juvenile', 2.0, '1-2 small crickets', 'Every 5-7 days', 'Small crickets', 'Slow growing species'),
(38, 'Sub-Adult', 4.0, '2 small crickets', 'Every 7-10 days', 'Small crickets, small roaches', 'Keep feeding moderate'),
(38, 'Adult', 9.0, '2-3 medium crickets', 'Every 14-21 days', 'Medium crickets, small roaches', 'Small adult size; dont overfeed'),

-- Homoeomma sp. blue (Peruvian Blue Bloom) - ID: 39
(39, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Moderate growth rate'),
(39, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Good feeding response'),
(39, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Regular feeder'),
(39, 'Adult', 12.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Maintains steady appetite'),

-- Thrixopelma ockerti (Peruvian Flame) - ID: 40
(40, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Good feeding response early'),
(40, 'Juvenile', 3.0, '2 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Steady growth rate'),
(40, 'Sub-Adult', 7.0, '2-3 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Consistent eater'),
(40, 'Adult', 13.0, '2-3 large crickets', 'Every 14-21 days', 'Large crickets, adult roaches', 'Regular feeding schedule');

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type, notes)
VALUES
-- Haploclastus devamatha (Indian Violet) - ID: 41
(41, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fossorial species; provide deep substrate'),
(41, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Will ambush from burrow'),
(41, 'Sub-Adult', 9.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Deep burrower; place prey near entrance'),
(41, 'Adult', 16.0, '3-4 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Use caution when feeding; defensive species'),

-- Chilobrachys fimbriatus (Indian Violet Tree Spider) - ID: 42
(42, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast and defensive from early age'),
(42, 'Juvenile', 3.5, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Heavy webber; place prey in web'),
(42, 'Sub-Adult', 8.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Will ambush from web tunnel'),
(42, 'Adult', 14.0, '3-4 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Very defensive; use long tongs'),

-- Poecilotheria metallica (Gooty Sapphire) - ID: 43
(43, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Delicate but eager feeder'),
(43, 'Juvenile', 3.5, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, flying insects', 'Place prey on cork bark'),
(43, 'Sub-Adult', 8.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, moths', 'Expert ambush hunter'),
(43, 'Adult', 15.0, '3-4 medium crickets', 'Every 14-21 days', 'Medium crickets, moths, flying insects', 'Very defensive; use extreme caution'),

-- Cyriopagopus sp. hati hati (Malaysian Earth Tiger) - ID: 44
(44, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Provide burrow from early age'),
(44, 'Juvenile', 4.0, '2-3 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Will create deep burrow'),
(44, 'Sub-Adult', 9.0, '3-4 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Aggressive burrow defender'),
(44, 'Adult', 16.0, '3-4 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Very defensive; use long tongs'),

-- Omothymus violaceopes (Singapore Blue) - ID: 45
(45, 'Spiderling', 0.5, 'Pre-killed pinhead cricket', '2-3 times per week', 'Pinhead crickets', 'Fast growing fossorial'),
(45, 'Juvenile', 4.5, '3-4 small crickets', 'Every 5-7 days', 'Small crickets, small roaches', 'Creates deep burrow system'),
(45, 'Sub-Adult', 11.0, '4-5 medium crickets', 'Every 7-10 days', 'Medium crickets, medium roaches', 'Strong feeding response'),
(45, 'Adult', 18.0, '4-5 medium crickets', 'Every 14-21 days', 'Medium crickets, adult roaches', 'Large defensive species; use caution'),

-- Hapalopus sp. Colombia (Pumpkin Patch) - ID: 46
(46, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Very small spiderling; careful with prey size'),
(46, 'Juvenile', 1.5, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Fast growing dwarf'),
(46, 'Sub-Adult', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Dont overfeed'),
(46, 'Adult', 7.0, '2-3 small crickets', 'Every 10-14 days', 'Small crickets, small roaches', 'Dwarf species; maintain small prey'),

-- Cyriocosmus elegans (Trinidad Dwarf Tiger) - ID: 47
(47, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Extremely small spiderling'),
(47, 'Juvenile', 1.0, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Very small species'),
(47, 'Sub-Adult', 3.0, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Keep prey size appropriate'),
(47, 'Adult', 6.0, '2-3 small crickets', 'Every 10-14 days', 'Small crickets, small roaches', 'One of smallest tarantula species'),

-- Neoholothele incei gold (Trinidad Olive Gold) - ID: 48
(48, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Tiny spiderling; careful feeding'),
(48, 'Juvenile', 1.5, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Heavy webber from young'),
(48, 'Sub-Adult', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Place prey in web'),
(48, 'Adult', 8.0, '2-3 small crickets', 'Every 10-14 days', 'Small crickets, small roaches', 'Dwarf species; maintain small prey'),

-- Homoeomma sp. peru (Peru Dwarf) - ID: 49
(49, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Very small spiderling'),
(49, 'Juvenile', 1.5, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Slow growing dwarf'),
(49, 'Sub-Adult', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Keep feeding moderate'),
(49, 'Adult', 8.0, '2-3 small crickets', 'Every 10-14 days', 'Small crickets, small roaches', 'Dwarf species; dont overfeed'),

-- Kochiana brunnipes (Brazilian Dwarf Beauty) - ID: 50
(50, 'Spiderling', 0.3, 'Pre-killed fruit fly', '3-4 times per week', 'Fruit flies', 'Extremely small spiderling'),
(50, 'Juvenile', 1.5, '1-2 small crickets', 'Every 4-5 days', 'Small crickets, fruit flies', 'Delicate dwarf species'),
(50, 'Sub-Adult', 3.5, '2 small crickets', 'Every 7 days', 'Small crickets, small roaches', 'Maintain appropriate prey size'),
(50, 'Adult', 7.0, '2-3 small crickets', 'Every 10-14 days', 'Small crickets, small roaches', 'True dwarf species; careful not to overfeed');