// Sample fitness and nutrition data for RAG system testing

pub struct FitnessSampleData;

impl FitnessSampleData {
    pub fn get_exercise_articles() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
        vec![
            (
                "The Science of Cardiovascular Exercise",
                "Cardiovascular exercise, commonly known as cardio, refers to any activity that raises your heart rate and keeps it elevated for an extended period. This type of exercise strengthens the heart muscle, improves circulation, and enhances the body's ability to deliver oxygen to working muscles.

The American Heart Association recommends at least 150 minutes of moderate-intensity aerobic activity or 75 minutes of vigorous-intensity aerobic activity per week for adults. Examples of moderate-intensity activities include brisk walking, water aerobics, and ballroom dancing. Vigorous activities include running, swimming laps, and cycling uphill.

Regular cardiovascular exercise provides numerous health benefits:
- Reduces risk of heart disease and stroke
- Lowers blood pressure and cholesterol levels
- Improves insulin sensitivity and glucose metabolism
- Enhances mental health and reduces symptoms of depression
- Increases bone density and reduces risk of osteoporosis
- Boosts immune system function
- Improves sleep quality and cognitive function

For beginners, it's important to start gradually and progressively increase intensity and duration. A typical progression might begin with 10-15 minutes of light activity three times per week, gradually building to longer sessions as fitness improves.",
                vec!["cardio", "heart-health", "aerobic", "exercise-guidelines"]
            ),

            (
                "Strength Training Fundamentals",
                "Resistance training, also known as strength training or weight training, involves exercises that cause muscles to contract against external resistance. This resistance can come from free weights, weight machines, resistance bands, or even body weight.

The principle of progressive overload is fundamental to strength training effectiveness. This means gradually increasing the stress placed on muscles over time through increases in weight, repetitions, sets, or exercise difficulty. Without progressive overload, muscles adapt to the current demands and cease to grow stronger.

Key principles of effective strength training:
1. Compound movements should form the foundation of any program. These exercises work multiple muscle groups simultaneously and include squats, deadlifts, bench press, and rows.
2. Training frequency should allow adequate recovery between sessions. Most muscle groups need 48-72 hours of rest between intense training sessions.
3. Proper form is crucial for both effectiveness and injury prevention. Quality of movement should never be compromised for heavier weight.
4. Rest periods between sets should be appropriate for the training goal: 1-2 minutes for endurance, 2-3 minutes for hypertrophy, and 3-5 minutes for strength.

Strength training benefits include increased muscle mass, improved bone density, enhanced metabolic rate, better functional movement patterns, and reduced risk of injury. Research shows that adults can expect to gain 2-4 pounds of muscle mass after 8-12 weeks of consistent resistance training.",
                vec!["strength-training", "resistance", "muscle-building", "progressive-overload"]
            ),

            (
                "High-Intensity Interval Training (HIIT)",
                "High-Intensity Interval Training (HIIT) is a cardiovascular exercise strategy that alternates short periods of intense anaerobic exercise with less intense recovery periods. A typical HIIT session lasts 10-30 minutes and can provide similar cardiovascular benefits to longer periods of moderate-intensity exercise.

The structure of HIIT workouts typically follows a work-to-rest ratio. Common ratios include:
- 1:1 ratio (30 seconds work, 30 seconds rest)
- 2:1 ratio (40 seconds work, 20 seconds rest)
- 1:2 ratio (15 seconds work, 45 seconds rest for very high intensity)

HIIT can be applied to various exercises including running, cycling, rowing, bodyweight exercises, and resistance training. The key is to reach 80-95% of maximum heart rate during work intervals, followed by active recovery at 40-50% maximum heart rate.

Physiological adaptations to HIIT training:
- Improved VO2 max (maximal oxygen uptake)
- Enhanced cardiac output and stroke volume
- Increased mitochondrial density and enzyme activity
- Improved insulin sensitivity and glucose metabolism
- Greater post-exercise oxygen consumption (EPOC), leading to continued calorie burn after exercise

HIIT offers several advantages over traditional steady-state cardio:
- Time efficiency - achieve similar benefits in less time
- Metabolic flexibility - improved ability to use both fat and carbohydrates for fuel
- Muscle preservation - less likely to cause muscle loss compared to long-duration cardio
- Variety and engagement - less monotonous than steady-state exercise

However, HIIT is demanding on the nervous system and joints, so it should be limited to 2-3 sessions per week with adequate recovery between sessions.",
                vec!["hiit", "interval-training", "cardiovascular", "time-efficient"]
            ),

            (
                "Exercise Recovery and Sleep",
                "Recovery is the period between workouts when the body adapts to the stress of exercise and replenishes energy stores. Without adequate recovery, performance plateaus or declines, and the risk of injury increases significantly.

Sleep is the most critical component of exercise recovery. During sleep, the body releases growth hormone, which is essential for tissue repair and muscle protein synthesis. The deep sleep stages (stages 3 and 4 of non-REM sleep) are particularly important for physical recovery.

Sleep recommendations for athletes and active individuals:
- 7-9 hours per night for adults
- Consistent sleep schedule, going to bed and waking at the same times
- Cool, dark, and quiet sleep environment
- Avoiding screens and caffeine 2-3 hours before bedtime
- Athletes may benefit from 8-10 hours of sleep during heavy training periods

Other recovery strategies include:
1. Active recovery: Light movement such as walking, gentle yoga, or swimming at very low intensity
2. Nutrition: Consuming adequate protein (20-40g) and carbohydrates within 2 hours post-exercise
3. Hydration: Replacing fluid losses from sweat and maintaining optimal hydration status
4. Stress management: Chronic psychological stress impairs recovery and adaptation
5. Massage and soft tissue work: Can improve circulation and reduce muscle tension

Signs of inadequate recovery include:
- Declining performance despite consistent training
- Elevated resting heart rate
- Mood disturbances and irritability
- Increased susceptibility to illness
- Persistent muscle soreness or joint stiffness
- Changes in appetite or sleep patterns

The principle of supercompensation explains why recovery is essential: after an exercise stimulus, performance initially decreases during the fatigue phase, then returns to baseline during recovery, and finally exceeds the original level if adequate rest is provided.",
                vec!["recovery", "sleep", "rest", "adaptation", "performance"]
            ),

            (
                "Flexibility and Mobility Training",
                "Flexibility refers to the passive range of motion available at a joint, while mobility encompasses both flexibility and the strength to control movement through that range of motion. Both are essential components of overall fitness and movement quality.

Types of stretching and their applications:
1. Static stretching: Holding a stretch for 15-60 seconds. Best performed after exercise when muscles are warm. Effective for improving flexibility but may temporarily reduce power output if performed before high-intensity activities.

2. Dynamic stretching: Controlled movements that take joints through their full range of motion. Ideal for warm-ups as it prepares the body for movement while maintaining muscle activation.

3. PNF stretching: Proprioceptive neuromuscular facilitation involves contracting and relaxing muscles to achieve greater range of motion. Often requires a partner and is highly effective for flexibility gains.

4. Active stretching: Using the strength of opposing muscles to move a joint through its range of motion. Develops both flexibility and strength simultaneously.

Common areas requiring attention in modern lifestyles:
- Hip flexors: Often tight from prolonged sitting
- Thoracic spine: Rounded posture from desk work and device use
- Shoulders: Internal rotation from forward head posture
- Ankles: Limited dorsiflexion from wearing shoes and sedentary behavior
- Hamstrings: Shortened from sitting and may compensate for weak glutes

Mobility exercises should address:
- Joint range of motion limitations
- Muscle length restrictions
- Movement pattern dysfunctions
- Strength through full range of motion

Benefits of regular flexibility and mobility work:
- Improved movement quality and efficiency
- Reduced risk of injury
- Better exercise performance
- Decreased muscle tension and stiffness
- Enhanced recovery between training sessions
- Improved posture and alignment

A comprehensive approach includes both daily mobility work (5-10 minutes) and dedicated flexibility sessions (20-30 minutes) 2-3 times per week.",
                vec!["flexibility", "mobility", "stretching", "range-of-motion", "movement-quality"]
            ),
        ]
    }

    pub fn get_nutrition_articles() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
        vec![
            (
                "Macronutrients for Athletic Performance",
                "Macronutrients - carbohydrates, proteins, and fats - are the foundation of sports nutrition and provide the energy and building blocks necessary for optimal athletic performance and recovery.

CARBOHYDRATES serve as the primary fuel source for high-intensity exercise. The body stores carbohydrates as glycogen in muscles and the liver, with total capacity ranging from 300-500g in trained individuals. During intense exercise, muscle glycogen can be depleted within 60-90 minutes.

Carbohydrate recommendations:
- Sedentary individuals: 3-5g per kg body weight daily
- Moderate exercise (1 hour/day): 5-7g per kg body weight
- Endurance training: 6-10g per kg body weight
- Ultra-endurance: 8-12g per kg body weight

Timing is crucial: consume carbohydrates 1-4 hours before exercise and within 30 minutes post-exercise for optimal glycogen replenishment.

PROTEIN is essential for muscle protein synthesis, immune function, and enzyme production. Complete proteins contain all nine essential amino acids that the body cannot produce independently.

Protein requirements:
- Sedentary adults: 0.8g per kg body weight
- Recreational athletes: 1.2-1.4g per kg
- Strength athletes: 1.6-2.2g per kg
- Endurance athletes: 1.2-1.6g per kg

Leucine, an essential amino acid, is particularly important for stimulating muscle protein synthesis. Aim for 2.5-3g of leucine per meal, found in approximately 25-30g of high-quality protein.

FATS provide essential fatty acids, support hormone production, and serve as a fuel source during low-to-moderate intensity exercise. Omega-3 fatty acids (EPA and DHA) have anti-inflammatory properties and support recovery.

Fat recommendations:
- 20-35% of total daily calories
- Emphasize monounsaturated and polyunsaturated fats
- Include omega-3 sources: fatty fish, flaxseeds, walnuts, chia seeds
- Limit saturated fat to less than 10% of total calories

Meal timing strategies:
- Pre-exercise: Focus on easily digestible carbohydrates with minimal fat and fiber
- During exercise: 30-60g carbohydrates per hour for sessions longer than 60 minutes
- Post-exercise: 3:1 or 4:1 carbohydrate to protein ratio within 2 hours",
                vec!["macronutrients", "carbohydrates", "protein", "fats", "sports-nutrition"]
            ),

            (
                "Hydration and Electrolyte Balance",
                "Proper hydration is critical for thermoregulation, cardiovascular function, and athletic performance. Even mild dehydration (2% body weight loss) can significantly impair both physical and cognitive performance.

Water functions in the body:
- Temperature regulation through sweating
- Transportation of nutrients and waste products
- Joint lubrication and shock absorption
- Maintenance of blood volume and pressure
- Cellular metabolism and chemical reactions

Factors affecting fluid needs:
- Exercise intensity and duration
- Environmental temperature and humidity
- Individual sweat rate (varies from 0.5-3.0 liters per hour)
- Acclimatization status
- Body size and composition
- Clothing and equipment

General hydration guidelines:
- Daily baseline: 35-40ml per kg body weight
- Pre-exercise: 400-600ml, 2-3 hours before activity
- During exercise: 150-250ml every 15-20 minutes
- Post-exercise: 150% of fluid losses (weigh before and after exercise)

ELECTROLYTES are minerals that carry an electrical charge and are essential for proper cellular function. The primary electrolytes lost in sweat are sodium and chloride, with smaller amounts of potassium, magnesium, and calcium.

Sodium functions:
- Maintains fluid balance between intracellular and extracellular spaces
- Essential for nerve impulse transmission
- Promotes carbohydrate and water absorption in the intestines
- Stimulates thirst mechanism

Signs of electrolyte imbalance:
- Muscle cramps or spasms
- Nausea or vomiting
- Headache or dizziness
- Confusion or altered mental status
- Fatigue or weakness

Electrolyte replacement strategies:
- For activities under 60 minutes: water is typically sufficient
- Activities over 60 minutes: sports drinks containing 6-8% carbohydrates and 200-700mg sodium per liter
- High-intensity or long-duration activities: may require 300-600mg sodium per hour
- Post-exercise: consume sodium with fluids to enhance retention

Natural electrolyte sources:
- Sodium: table salt, sea salt, salted nuts
- Potassium: bananas, oranges, potatoes, coconut water
- Magnesium: leafy greens, nuts, seeds, whole grains
- Calcium: dairy products, leafy greens, fortified foods

Environmental considerations:
- Hot, humid conditions increase sweat rate and sodium losses
- Altitude can increase fluid needs due to increased respiratory losses
- Air conditioning and heating can affect hydration status
- Alcohol and caffeine have mild diuretic effects",
                vec!["hydration", "electrolytes", "sodium", "fluid-balance", "thermoregulation"]
            ),

            (
                "Pre and Post-Workout Nutrition",
                "Strategic nutrient timing around exercise can enhance performance, accelerate recovery, and optimize training adaptations. The timing, composition, and quantity of meals and snacks significantly impact energy availability and recovery processes.

PRE-EXERCISE NUTRITION goals:
- Maximize glycogen stores
- Provide readily available energy
- Maintain stable blood sugar levels
- Minimize gastrointestinal distress
- Optimize hydration status

Timing considerations:
3-4 hours before: Large meal with 1-4g carbohydrates per kg body weight, moderate protein, low fat and fiber
2-3 hours before: Smaller meal with 1-2g carbohydrates per kg body weight
1 hour before: Small snack with 0.5-1g carbohydrates per kg body weight
15-30 minutes before: Easily digestible carbohydrates (banana, dates, sports drink)

Pre-workout meal examples:
- Oatmeal with banana and berries
- Greek yogurt with granola and fruit
- Toast with honey and a small amount of nut butter
- Smoothie with fruit, yogurt, and a small amount of protein powder

Foods to avoid pre-exercise:
- High-fat foods (slow gastric emptying)
- High-fiber foods (may cause GI distress)
- New or unfamiliar foods
- Large quantities of protein (not immediately useful for energy)

POST-EXERCISE NUTRITION priorities:
1. Glycogen replenishment
2. Muscle protein synthesis stimulation
3. Rehydration
4. Anti-inflammatory support

The post-exercise 'anabolic window' is longer than traditionally thought (up to 6 hours), but consuming nutrients within 2 hours optimizes recovery, especially when the next training session is within 8 hours.

Carbohydrate recommendations post-exercise:
- Immediate (0-30 minutes): 0.5-1.2g per kg body weight
- Extended recovery (0-6 hours): 1.2-1.7g per kg body weight per hour
- Choose high glycemic index foods for rapid glycogen resynthesis

Protein recommendations post-exercise:
- 20-40g of high-quality protein
- Include 2.5-3g of leucine to maximize muscle protein synthesis
- Casein protein may be beneficial before sleep for overnight recovery

Optimal post-workout combinations:
- Chocolate milk (3:1 or 4:1 carb to protein ratio)
- Greek yogurt with berries and granola
- Turkey and avocado sandwich
- Protein smoothie with fruit
- Quinoa bowl with vegetables and lean protein

Hydration post-exercise:
- Weigh yourself before and after exercise
- Drink 150% of weight lost (1.5 liters per kg of weight loss)
- Include sodium to enhance fluid retention
- Monitor urine color as a hydration indicator

Special considerations:
- Morning exercisers: focus on easily digestible carbohydrates
- Evening exercisers: avoid excessive caffeine that may interfere with sleep
- Multiple daily sessions: prioritize rapid recovery nutrition
- Weight loss goals: moderate post-exercise nutrition but don't skip it entirely",
                vec!["pre-workout", "post-workout", "nutrient-timing", "recovery-nutrition", "glycogen"]
            ),

            (
                "Supplements for Fitness and Performance",
                "While a well-balanced diet should provide most nutrients needed for health and performance, certain supplements have strong scientific evidence for enhancing exercise performance, recovery, and overall health in active individuals.

EVIDENCE-BASED SUPPLEMENTS:

Creatine Monohydrate:
- Most researched sports supplement with over 1000 studies
- Increases phosphocreatine stores in muscles, enhancing power output for short-duration, high-intensity activities
- Dosing: 3-5g daily, timing not critical
- Benefits: increased strength, power, muscle mass, and recovery
- Safe for long-term use with no significant side effects
- May cause initial water retention (1-2kg weight gain)

Protein Powder:
- Convenient way to meet daily protein requirements
- Whey protein: fast-absorbing, high in leucine, ideal post-workout
- Casein protein: slow-absorbing, good before sleep
- Plant proteins: suitable for vegans, may require larger amounts
- Dosing: 20-40g per serving as needed to meet daily protein goals

Caffeine:
- Enhances alertness, reduces perceived exertion, and improves endurance performance
- Effective dose: 3-6mg per kg body weight, 30-60 minutes before exercise
- Benefits peak at 45-75 minutes post-consumption
- Individual tolerance varies; start with lower doses
- Avoid within 6 hours of sleep to prevent sleep disruption

Beta-Alanine:
- Increases muscle carnosine levels, buffering acid in muscles
- Improves performance in high-intensity exercise lasting 1-4 minutes
- Dosing: 3-5g daily, divided into smaller doses with meals
- May cause harmless tingling sensation (paresthesia)
- Benefits accumulate over 2-4 weeks of consistent use

CONDITIONALLY BENEFICIAL SUPPLEMENTS:

Citrulline Malate:
- May improve blood flow and reduce muscle soreness
- Dosing: 6-8g before exercise
- More research needed for definitive recommendations

HMB (β-Hydroxy β-Methylbutyrate):
- May reduce muscle protein breakdown
- Most beneficial for untrained individuals or during periods of high stress
- Dosing: 3g daily in divided doses with meals

Branched-Chain Amino Acids (BCAAs):
- May reduce muscle soreness and fatigue
- Less beneficial if adequate protein intake is achieved through diet
- More useful during caloric restriction or fasted training

SUPPLEMENTS WITH LIMITED EVIDENCE:

Glutamine:
- Often marketed for recovery and immune function
- Healthy individuals typically produce adequate amounts
- Benefits mainly seen in clinical populations or extreme stress

Testosterone Boosters:
- Most natural testosterone boosters have minimal effect on healthy individuals
- Focus on adequate sleep, nutrition, and resistance training for natural optimization

SAFETY CONSIDERATIONS:
- Third-party testing: Look for NSF, Informed Sport, or USP certification
- Start with single ingredients rather than proprietary blends
- Consult healthcare providers, especially if taking medications
- Be aware of banned substances if competing in tested sports
- Remember supplements supplement, they don't replace, a good diet

TIMING STRATEGIES:
- Creatine: anytime, consistency matters more than timing
- Protein: post-workout and throughout the day to meet targets
- Caffeine: 30-60 minutes before exercise
- Beta-alanine: with meals to minimize tingling sensation",
                vec!["supplements", "creatine", "protein-powder", "caffeine", "performance-enhancement"]
            ),

            (
                "Weight Management and Body Composition",
                "Achieving and maintaining optimal body composition requires understanding energy balance, metabolic adaptations, and the interplay between diet, exercise, and lifestyle factors. Successful long-term weight management goes beyond simple calorie counting.

ENERGY BALANCE FUNDAMENTALS:
Energy balance = Energy intake - Energy expenditure

Components of total daily energy expenditure (TDEE):
1. Basal Metabolic Rate (BMR): 60-75% of TDEE
   - Energy required for basic physiological functions
   - Influenced by age, sex, body size, muscle mass, genetics

2. Thermic Effect of Food (TEF): 8-10% of TDEE
   - Energy cost of digesting, absorbing, and processing food
   - Protein has highest TEF (~20-30%), followed by carbs (~5-10%) and fats (~0-3%)

3. Exercise Activity Thermogenesis (EAT): 15-30% of TDEE
   - Structured, planned physical activities

4. Non-Exercise Activity Thermogenesis (NEAT): 15-50% of TDEE
   - All activities outside of sleeping, eating, or sports-like exercise
   - Includes fidgeting, maintaining posture, daily activities

METABOLIC ADAPTATIONS:
During caloric restriction, the body employs several adaptive mechanisms:
- Decreased BMR (up to 15-20% below predicted values)
- Reduced NEAT and spontaneous movement
- Increased hunger hormones (ghrelin) and decreased satiety hormones (leptin)
- Improved metabolic efficiency
- Preferential storage of incoming calories as fat

These adaptations explain why weight loss often plateaus and why weight regain is common.

STRATEGIES FOR FAT LOSS:
1. Create a moderate caloric deficit (300-750 calories below maintenance)
2. Prioritize protein intake (1.6-2.2g per kg body weight)
3. Include resistance training to preserve muscle mass
4. Incorporate both cardio and strength training
5. Focus on whole, minimally processed foods
6. Stay adequately hydrated
7. Prioritize sleep quality and duration
8. Manage stress levels

MUSCLE BUILDING (HYPERTROPHY):
Requirements for muscle growth:
- Progressive overload in resistance training
- Adequate protein intake (1.6-2.2g per kg body weight)
- Sufficient calories to support growth (slight surplus)
- Adequate sleep and recovery
- Consistency over time (months to years)

Muscle building is a slow process:
- Beginners: 1-2 pounds muscle per month
- Intermediate: 0.5-1 pound muscle per month
- Advanced: 0.25-0.5 pounds muscle per month

BODY RECOMPOSITION:
Simultaneous fat loss and muscle gain is possible but challenging:
- Most effective in beginners or returning trainees
- Requires precise nutrition and training protocols
- Focus on strength training with adequate protein
- Modest caloric deficit or maintenance calories
- Patient approach as changes occur slowly

SUSTAINABLE PRACTICES:
- Gradual changes rather than extreme restrictions
- Focus on habits and behaviors, not just outcomes
- Include foods you enjoy in moderation
- Plan for social situations and travel
- Regular monitoring without obsession
- Flexible approach that adapts to life changes

COMMON MISTAKES:
- Extreme caloric restrictions leading to metabolic damage
- Eliminating entire food groups unnecessarily
- Focusing solely on scale weight rather than body composition
- Neglecting strength training during weight loss
- All-or-nothing mentality
- Ignoring the importance of sleep and stress management

MONITORING PROGRESS:
- Scale weight (daily weigh-ins, weekly averages)
- Body measurements (waist, hips, arms, thighs)
- Progress photos in consistent conditions
- Performance metrics (strength, endurance)
- How clothes fit and energy levels
- Body composition testing (DEXA, BodPod) if available",
                vec!["weight-management", "body-composition", "fat-loss", "muscle-building", "metabolism"]
            ),
        ]
    }

    pub fn get_all_sample_documents() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
        let mut all_docs = Self::get_exercise_articles();
        all_docs.extend(Self::get_nutrition_articles());
        all_docs
    }

    pub fn get_sample_queries() -> Vec<(&'static str, Vec<&'static str>)> {
        vec![
            (
                "How much cardio should I do per week?",
                vec!["cardiovascular", "cardio", "heart-health", "exercise-guidelines"]
            ),
            (
                "What are the benefits of strength training?",
                vec!["strength-training", "resistance", "muscle-building"]
            ),
            (
                "How do I build muscle effectively?",
                vec!["strength-training", "protein", "muscle-building", "progressive-overload"]
            ),
            (
                "What should I eat before a workout?",
                vec!["pre-workout", "nutrition", "carbohydrates"]
            ),
            (
                "How much protein do I need daily?",
                vec!["protein", "macronutrients", "sports-nutrition"]
            ),
            (
                "What is HIIT and how does it work?",
                vec!["hiit", "interval-training", "cardiovascular"]
            ),
            (
                "How important is sleep for recovery?",
                vec!["recovery", "sleep", "rest", "adaptation"]
            ),
            (
                "Should I take creatine supplements?",
                vec!["supplements", "creatine", "performance-enhancement"]
            ),
            (
                "How do I lose fat while keeping muscle?",
                vec!["weight-management", "fat-loss", "muscle-building", "body-composition"]
            ),
            (
                "Why is hydration important during exercise?",
                vec!["hydration", "electrolytes", "thermoregulation"]
            ),
            (
                "What stretches should I do after working out?",
                vec!["flexibility", "mobility", "stretching", "recovery"]
            ),
            (
                "How do I prevent muscle cramps?",
                vec!["hydration", "electrolytes", "sodium", "muscle-cramps"]
            ),
        ]
    }
}