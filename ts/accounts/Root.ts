import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RootFields {
  /** The ID of the root */
  id: PublicKey
  /** The token used to vote for a tag */
  voteMint: PublicKey
  /** Admin of the root */
  admin: PublicKey
  /** Cost to create a tree from this root */
  treeCreationFee: BN
}

export interface RootJSON {
  /** The ID of the root */
  id: string
  /** The token used to vote for a tag */
  voteMint: string
  /** Admin of the root */
  admin: string
  /** Cost to create a tree from this root */
  treeCreationFee: string
}

export class Root {
  /** The ID of the root */
  readonly id: PublicKey
  /** The token used to vote for a tag */
  readonly voteMint: PublicKey
  /** Admin of the root */
  readonly admin: PublicKey
  /** Cost to create a tree from this root */
  readonly treeCreationFee: BN

  static readonly discriminator = Buffer.from([46, 159, 131, 37, 245, 84, 5, 9])

  static readonly layout = borsh.struct([
    borsh.publicKey("id"),
    borsh.publicKey("voteMint"),
    borsh.publicKey("admin"),
    borsh.u64("treeCreationFee"),
  ])

  constructor(fields: RootFields) {
    this.id = fields.id
    this.voteMint = fields.voteMint
    this.admin = fields.admin
    this.treeCreationFee = fields.treeCreationFee
  }

  static async fetch(c: Connection, address: PublicKey): Promise<Root | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<Root | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): Root {
    if (!data.slice(0, 8).equals(Root.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Root.layout.decode(data.slice(8))

    return new Root({
      id: dec.id,
      voteMint: dec.voteMint,
      admin: dec.admin,
      treeCreationFee: dec.treeCreationFee,
    })
  }

  toJSON(): RootJSON {
    return {
      id: this.id.toString(),
      voteMint: this.voteMint.toString(),
      admin: this.admin.toString(),
      treeCreationFee: this.treeCreationFee.toString(),
    }
  }

  static fromJSON(obj: RootJSON): Root {
    return new Root({
      id: new PublicKey(obj.id),
      voteMint: new PublicKey(obj.voteMint),
      admin: new PublicKey(obj.admin),
      treeCreationFee: new BN(obj.treeCreationFee),
    })
  }
}
