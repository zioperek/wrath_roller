pub enum Attribute {
    Strength,
    Agility,
    Toughness,
    Intellect,
    Willpower,
    Fellowship,
    Initiative

}

pub enum Skill {
    Athletics,
    Awareness,
    BallisticSkill,
    Cunning,
    Deception,
    Insight,
    Intimidation,
    Investigation,
    Leadership,
    Medicae,
    Persuasion,
    Pilot,
    PsychicMastery,
    Scholar,
    Stealth,
    Survival,
    Tech,
    WeaponSkill,
}

#[derive(Default, Debug)]
pub struct Attributes {
    pub strength: u8,
    pub agility: u8,
    pub toughness: u8,
    pub intellect: u8,
    pub willpower: u8,
    pub fellowship: u8,
    pub initiative: u8
}

#[derive(Default, Debug)]
pub struct Skills {
    pub athletics: u8,
    pub awareness: u8,
    pub ballistic_skill: u8,
    pub cunning: u8,
    pub deception: u8,
    pub insight: u8,
    pub intimidation: u8,
    pub investigation: u8,
    pub leadership: u8,
    pub medicae: u8,
    pub persuasion: u8,
    pub pilot: u8,
    pub psychic_mastery: u8,
    pub scholar: u8,
    pub stealth: u8,
    pub survival: u8,
    pub tech: u8,
    pub weapon_skill: u8,
}


impl Skill {
    pub fn linked_attributes(&self) -> Attribute {
        match self {
            Skill::Athletics => Attribute::Strength,
            Skill::Awareness => Attribute::Intellect,
            Skill::BallisticSkill => Attribute::Agility,
            Skill::Cunning => Attribute::Fellowship,
            Skill::Deception => Attribute::Fellowship,
            Skill::Insight => Attribute::Fellowship,
            Skill::Intimidation => Attribute::Willpower,
            Skill::Investigation => Attribute::Intellect,
            Skill::Leadership => Attribute::Willpower,
            Skill::Medicae => Attribute::Intellect,
            Skill::Persuasion => Attribute::Fellowship,
            Skill::Pilot => Attribute::Agility,
            Skill::PsychicMastery => Attribute::Willpower,
            Skill::Scholar => Attribute::Intellect,
            Skill::Stealth => Attribute::Agility,
            Skill::Survival => Attribute::Willpower,
            Skill::Tech => Attribute::Intellect,
            Skill::WeaponSkill => Attribute::Intellect,
        }
    }
}

#[derive(Debug)]
pub struct Character {
    pub attributes: Attributes,
    pub skills: Skills,
}


impl Character {
    pub fn new(attributes: Attributes, skills: Skills) -> Self {
        Self { attributes, skills }
    }
    pub fn total_for_skill(&self, skill: Skill) -> u8 {
        let skill_val = match skill {
            Skill::Athletics => self.skills.athletics,
            Skill::Awareness => self.skills.awareness,
            Skill::BallisticSkill => self.skills.ballistic_skill,
            Skill::Cunning => self.skills.cunning,
            Skill::Deception => self.skills.deception,
            Skill::Insight => self.skills.insight,
            Skill::Intimidation => self.skills.intimidation,
            Skill::Investigation => self.skills.investigation,
            Skill::Leadership => self.skills.leadership,
            Skill::Medicae => self.skills.medicae,
            Skill::Persuasion => self.skills.persuasion,
            Skill::Pilot => self.skills.pilot,
            Skill::PsychicMastery => self.skills.psychic_mastery,
            Skill::Scholar => self.skills.scholar,
            Skill::Stealth => self.skills.stealth,
            Skill::Survival => self.skills.survival,
            Skill::Tech => self.skills.tech,
            Skill::WeaponSkill => self.skills.weapon_skill,
        };

        let attr_val = match skill.linked_attributes() {
            Attribute::Strength => self.attributes.strength,
            Attribute::Agility => self.attributes.agility,
            Attribute::Toughness => self.attributes.toughness,
            Attribute::Intellect => self.attributes.intellect,
            Attribute::Willpower => self.attributes.willpower,
            Attribute::Fellowship => self.attributes.fellowship,
            Attribute::Initiative => self.attributes.initiative,
        };

        skill_val + attr_val
    }
}

pub fn make_dummy() {
    let mut attrs = Attributes::default();
    attrs.strength = 3;
    attrs.intellect = 5;
    attrs.agility = 5;

    let mut skills = Skills::default();
    skills.awareness = 5;
    skills.pilot = 5;

    let c = Character::new(attrs, skills);
    dbg!(c.total_for_skill(Skill::Pilot));
}