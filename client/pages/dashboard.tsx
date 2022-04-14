import { Center, Container, TextInput, Button, Title, Table } from "@mantine/core"
import { NextPage } from "next"
import { useEffect, useState } from "react"
import Service from "../services/transaction"
import { Block } from "../types/block"
import styles from "../styles/Dashboard.module.css"

const GambleForm = () => {
    const [value, setVal]: [string, Function] = useState('')
    const [balance, setBalance]: [Number, Function] = useState(0) // recoil dispatch
    useEffect(() => {
        getBlanance()
    }, [])
    const getBlanance = async () => {
        let balanceData = await Service.getBlanance()
        setBalance(parseInt(balanceData.balance))
    }
    return (
        <>
            <code>
                <Title className={styles.text} order={3}>Balance: {balance.toString()}</Title>
                <br />
                <TextInput
                    placeholder="Amount"
                    label="Gamble with Night Coins"
                    value={value}
                    onChange={(e) => setVal(e.currentTarget.value)}
                />
            </code>
            <br />
            <Button className={styles.text} variant="gradient" gradient={{ from: 'teal', to: 'blue', deg: 60 }}>Gamble</Button>
        </>
    )
}

const TransactionComponent = () => {
    // listen to add transaction
    let [page, setPage]: [Number, Function] = useState(0)
    let [transactions, setTransaction]: [Block[], Function] = useState([])
    useEffect(() => {
        getTransactions()
    }, [])
    const getTransactions = async () => {
        let elements: Block[] = await Service.getTransactions(page)
        setTransaction(elements)
    }

    const rows = transactions.map((element) => (
        <tr key={element.id.toString()}>
            <td>{element.id.toString()}</td>
            <td>{element.transaction.sender}</td>
            <td>{element.transaction.reciever}</td>
            <td>${element.transaction.amount.toString()}</td>
        </tr>
    ))

    console.log(transactions)
    return (
        <div>
            <h1 style={{marginTop: 5 + `rem`}} className={styles.text}>Transactions</h1>
            <Table className={styles.text} style={{ textAlign: "center" }}>
                <thead>
                    <tr >
                        <th>ID</th>
                        <th>Sender</th>
                        <th>Reciever</th>
                        <th>Amount</th>
                    </tr>
                </thead>
                <tbody>{rows}</tbody>
            </Table>
        </div>
    )
}

const Dashboard: NextPage = () => {
    return (
        <Center>
            <div className={styles.container}>
                <Container>
                    <h1 className={styles.heading}>Dashboard</h1>
                    <GambleForm />
                    {/* spacer */}
                    <TransactionComponent />
                </Container>
            </div>
        </Center>
    )
}

export default Dashboard