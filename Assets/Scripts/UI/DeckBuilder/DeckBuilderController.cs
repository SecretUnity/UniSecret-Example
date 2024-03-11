using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class DeckBuilderController : MonoBehaviour
{
    [SerializeField] private GameObject tilePrefab;
    [SerializeField] private LayoutGroup cardsPanel;
    [SerializeField] private CardTile preview;

    private SecretLoader secretLoader;
    private CardTile[] cards;

    // Start is called before the first frame update
    async void Start()
    {
        secretLoader = SecretLoader.Instance;
        var cards = await secretLoader.QueryContractState<CardTemplate[]>(
            "secret1zag3hdz0e0aqnw9450dawg7j6j56uww8xxhqrn",
            new
            {
                owned_cards = new
                {
                    address = secretLoader.Signer.Address
                }
            }
        );
        LoadCards(cards.Response);
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyUp(KeyCode.Escape))
            GoToMainMenu();
    }

    private void LoadCards(CardTemplate[] cards)
    { 
        var list = new List<CardTile>();
        foreach (CardTemplate card in cards)
        {
            var tile = Instantiate(tilePrefab, cardsPanel.transform).GetComponent<CardTile>();
            tile.Assign(card);
            var entry = new EventTrigger.Entry();
            entry.callback.AddListener((a) =>
            {
                preview.Assign(card);
            });
            entry.eventID = EventTriggerType.PointerEnter;
            tile.GetComponent<EventTrigger>().triggers.Add(entry);
            list.Add(tile);
        }
        this.cards = list.ToArray();
        foreach (var card in this.cards)
            card.UpdateUI();
    }

    public void GoToMainMenu()
    {
        SceneManager.LoadScene("MainMenu");
    }

}
