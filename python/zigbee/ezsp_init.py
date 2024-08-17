import sys
import traceback
import asyncio
import logging
from bellows.ezsp import EZSP
from zigpy.config import CONF_DEVICE, CONF_DEVICE_PATH, CONF_DEVICE_FLOW_CONTROL, CONF_DEVICE_BAUDRATE, CONF_NWK_EXTENDED_PAN_ID, CONF_NWK_PAN_ID, CONF_NWK_KEY
from bellows.config import CONF_USE_THREAD
from bellows.types import EmberNetworkParameters, EmberNetworkStatus, EmberJoinMethod
from bellows.exception import EzspError

logging.basicConfig(level=logging.INFO)
logging.getLogger("bellows").setLevel(logging.DEBUG)
logger = logging.getLogger(__name__)

async def init():
    device_path = "/dev/sonoff-zbdongle"
    config = {
        CONF_DEVICE: {
            CONF_DEVICE_PATH: device_path,
            CONF_DEVICE_FLOW_CONTROL: "software",
            CONF_DEVICE_BAUDRATE: 230400,
        },
        CONF_USE_THREAD: False
    }
    try:
        ezsp = await EZSP.initialize(config)
        await ezsp.version()
        logger.info("EZSP connected successfully")
    except EzspError as e:
        logger.error(f"Failed to connect to EZSP: {e}")
        raise

    return ezsp


async def init_network(ezsp):
    network_params = EmberNetworkParameters(
        panId=0x1234,
        extendedPanId=bytes.fromhex("DEADBEEF00000000"),
        radioTxPower=8,
        radioChannel=15,
        joinMethod=EmberJoinMethod.USE_MAC_ASSOCIATION,
        nwkManagerId=0x0000,
        nwkUpdateId=0,
        channels=0x07FFF800,
    )

    try:
        status = await ezsp.formNetwork(network_params)
        if status != EmberStatus.SUCCESS:
            print(f"Network formation failed: {status}")
    except EzspError as e:
        logger.error(f"Error during network formation: {e}")


async def main():
    try:
        ezsp = await init()
        await init_network(ezsp)
    except Exception as e:
        logger.error(f"An error occured: {e}")
        traceback.print_exc()
    finally:
        if "ezsp" in locals() and type(ezsp) is EZSP:
            ezsp.close()


if __name__ == "__main__":
    asyncio.run(main())
