using SecretNET.Tx;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class MainMenuController : MonoBehaviour
{
    private SecretLoader secretLoader;

    [SerializeField] private Button welcomePackRewardsButton;
    // Start is called before the first frame update
    async void Start()
    {
        secretLoader = SecretLoader.Instance;
        Debug.Log(secretLoader.Signer.Address);
        var welcomePack = await secretLoader.QueryContractState<WelcomePackQuery>(
            "secret1zag3hdz0e0aqnw9450dawg7j6j56uww8xxhqrn", 
            new 
            { 
                qualified_for_welcome_pack = new 
                { 
                    address = secretLoader.Signer.Address 
                } 
            }
        );
        Debug.Log(welcomePack.RawResponse);
        welcomePackRewardsButton.enabled = welcomePack.Response.Qualified;
    }

    // Update is called once per frame
    void Update()
    {
    }

    public async void ClaimWelcomePack()
    {
        await secretLoader.SignTransaction(
            new[] { 
                new MsgExecuteContract(
                    "secret1zag3hdz0e0aqnw9450dawg7j6j56uww8xxhqrn", 
                    new { receive_welcome_pack = new { } }) 
                {
                    Sender = secretLoader.Signer.Address 
                }
            }
        );
    }

    public async void BuyBoosterPack()
    {
        await secretLoader.SignTransaction(new[] { new MsgExecuteContract("secret1zag3hdz0e0aqnw9450dawg7j6j56uww8xxhqrn", new { buy_booster_pack = new { } }) { Sender = secretLoader.Signer.Address } });
    }

    public void OpenDeckBuilder()
    {
        SceneManager.LoadScene("DeckBuilder");
    }
}
