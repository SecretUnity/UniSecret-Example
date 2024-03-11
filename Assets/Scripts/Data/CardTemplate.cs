using Newtonsoft.Json;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using UnityEngine;

public class CardTemplate
{
    public string Name { get; set; }
    public string Description { get; set; }
    public string Image { get; set; }
    [JsonProperty("card_type")]
    public CardType CardType { get; set; } // Assuming CardType is an enum or class defined elsewhere
}

public class CardType
{
    public CaptainData Captain { get; set; }
    public ShipData Ship { get; set; }
    public CrewData Crew { get; set; }
    public SkillData Skill { get; set; }

    public override string ToString()
    {
        if (Captain != null)
            return nameof(Captain);
        else if (Ship != null) return nameof(Ship);
        else if (Crew != null) return nameof(Crew);
        else if (Skill != null) return nameof(Skill);
        else return "N/A";
    }
}

public class CaptainData
{
    public byte Damage { get; set; }
    public byte Health { get; set; }
    public byte Luck { get; set; }
}

public class ShipData
{
    public byte Durability { get; set; }
    public byte Speed { get; set; }
}

public class CrewData
{
    public byte Damage { get; set; }
    public byte Health { get; set; }
    public Ability[] Abilities { get; set; }
}

public class Ability
{

}

public class SkillData
{
    public AbilityEffect Effect { get; set; }
}

public class AbilityEffect
{

} 
