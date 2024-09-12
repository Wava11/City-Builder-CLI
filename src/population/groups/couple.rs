use crate::{housing::{get_housing_capacity, get_housing_type, Housing, HousingType}, population::{CitizenBundle, ScoreHousing, ToCitizensBundles}, statistics::{Sample, SampleBasedOn}};

struct Couple {
    members: [CitizenBundle; 2],
}
impl ToCitizensBundles for Couple {
    fn to_citizens_bundles(&self) -> Vec<CitizenBundle> {
        self.members.clone().into()
    }
}

impl ScoreHousing for Couple {
    fn score_housing<'a>(&self, housing: &Housing<'a>) -> u16 {
        if get_housing_capacity(housing) < 2 {
            return 0;
        } 
        match get_housing_type(housing) {
            HousingType::SingleFamilyHome => u16::MAX,
            HousingType::Apartment => u16::MAX/2,
        }
    }
}

pub struct CoupleDistribution {
    // rng: ThreadRng,
    // min_age: u8,
    // max_age: u8,
    // age_distribution: Normal<f32>,
    members_distribution: Box<dyn Sample<CitizenBundle>>,
    parter_distribution: Box<dyn SampleBasedOn<CitizenBundle>>,
}
impl CoupleDistribution {
    fn new(
        // min_age: u8,
        // max_age: u8,
        // mean_age: f32,
        // std_age: f32,
        members_distribution: Box<dyn Sample<CitizenBundle>>,
        parter_distribution: Box<dyn SampleBasedOn<CitizenBundle>>,
    ) -> Self {
        Self {
            // rng: rand::thread_rng(),
            // min_age,
            // max_age,
            // age_distribution: Normal::new(mean_age, std_age).unwrap(),
            members_distribution,
            parter_distribution,
        }
    }
}
impl Sample<Couple> for CoupleDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Couple> {
        (0..amount)
            .map(|_| {
                let first_member = self
                    .members_distribution
                    .sample(1)
                    .into_iter()
                    .next()
                    .unwrap();
                let second_member = self.parter_distribution.sample_based_on(&first_member);
                Couple {
                    members: [first_member, second_member],
                }
            })
            .collect()
    }
}
