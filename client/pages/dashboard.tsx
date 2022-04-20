import {
  Center,
  Container,
  TextInput,
  Button,
  Title,
  Table,
  Pagination,
} from "@mantine/core";
import { NextPage } from "next";
import { useEffect, useState, Dispatch, SetStateAction } from "react";
import Service from "../services/transaction";
import { Block } from "../types/block";
import { GambleData } from "../types/gamble";
import styles from "../styles/Dashboard.module.css";
import { useNotifications } from "@mantine/notifications";
const GambleForm = ({
  setTransactionEvent,
  transactionEvent,
}: {
  setTransactionEvent: Dispatch<SetStateAction<number>>;
  transactionEvent: number;
}) => {
  const [value, setVal] = useState("");
  const [balance, setBalance] = useState(0);
  const { showNotification } = useNotifications();
  useEffect(() => {
    getBlanance();
  }, []);
  const getBlanance = async () => {
    let balanceData = await Service.getBlanance();
    setBalance(parseInt(balanceData.balance));
  };
  const gamble = async () => {
    if (!isNaN(parseInt(value))) {
      let res: GambleData = await Service.gamble(parseInt(value));
      if (res.win == "true") {
        showNotification({
          message: "You won",
          color: "green",
        });
      } else {
        showNotification({
          message: "You lost",
          color: "red",
        });
      }
      setBalance(res.newBalance);
    } else {
      alert("Wrong Gamble amount");
    }
    setTransactionEvent(transactionEvent + 1);
  };
  return (
    <>
      <Title className={styles.text} order={3}>
        Balance: {balance.toString()}
      </Title>
      <br />
      <TextInput
        placeholder="Amount"
        label="Gamble with Night Coins"
        value={value}
        onChange={(e) => setVal(e.currentTarget.value)}
        required
      />
      <br />
      <Button
        onClick={gamble}
        className={styles.text}
        variant="gradient"
        gradient={{ from: "teal", to: "blue", deg: 60 }}
      >
        Gamble
      </Button>
    </>
  );
};

const TransactionComponent = ({ transactionEvent }: { transactionEvent: number }) => {
  let [dataLength, setDataLength] = useState(0);
  let [page, setPage] = useState(0);
  let [transactions, setTransaction] = useState<Block[]>([]);
  useEffect(() => {
    getDataLength();
  }, [transactionEvent]);
  useEffect(() => {
    getTransactions();
  }, [page, transactionEvent]);
  const getTransactions = async () => {
    if (page == 1 || page == 0) {
      let elements: Block[] = await Service.getTransactions(1);
      setTransaction(elements);
    } else {
      let elements: Block[] = await Service.getTransactions(
        dataLength - page + 2
      );
      setTransaction(elements);
    }
  };
  const getDataLength = async () => {
    let dataLength = await Service.countDataLength();
    setDataLength(dataLength);
  };
  const rows = transactions.map((element) => (
    <tr key={element.id.toString()}>
      <td>{element.block_hash.substring(0, 8)}</td>
      <td>{element.transaction.sender}</td>
      <td>{element.transaction.reciever}</td>
      <td>${element.transaction.amount.toString()}</td>
    </tr>
  ));
  return (
    <div>
      <h1 style={{ marginTop: 5 + `rem` }} className={styles.text}>
        Transactions
      </h1>
      <br />
      <br />
      <Table className={styles.text}>
        <thead>
          <tr>
            <th>Hash</th>
            <th>Sender</th>
            <th>Reciever</th>
            <th>Amount</th>
          </tr>
        </thead>
        <tbody>{rows}</tbody>
      </Table>
      <br />
      <br />
      <Center>
        <Pagination
          onChange={setPage}
          total={dataLength}
          siblings={1}
          initialPage={1}
        />
      </Center>
      <br />
    </div>
  );
};
const Dashboard: NextPage = () => {
  const [transactionEvent, setTransactionEvent] = useState(0);

  return (
    <Center>
      <div className={styles.container}>
        <Container>
          <h1 className={styles.heading}>Dashboard</h1>
          <GambleForm {...{ setTransactionEvent, transactionEvent }} />
          <TransactionComponent {...{ transactionEvent }} />
        </Container>
      </div>
    </Center>
  );
};
export default Dashboard;
