using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class CardTile : MonoBehaviour
{
    [SerializeField] private TextMeshProUGUI titleField;
    [SerializeField] private TextMeshProUGUI typeField;
    [SerializeField] private TextMeshProUGUI descriptionField;

    private CardTemplate cardTemplate;

    public void Assign(CardTemplate card)
    {
        cardTemplate = card;
    }

    public void UpdateUI()
    {
        titleField.SetText(cardTemplate.Name);
        descriptionField.SetText(cardTemplate.Description);
        typeField.SetText(cardTemplate.CardType.ToString());
    }
}
