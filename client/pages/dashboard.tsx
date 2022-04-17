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
  setTransaction,
  transaction,
}: {
  setTransaction: Dispatch<SetStateAction<number>>;
  transaction: number;
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
    setTransaction(transaction + 1);
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

const TransactionComponent = ({ transaction }: { transaction: number }) => {
  let [dataLength, setDataLength] = useState(0);
  let [page, setPage] = useState(1);
  let [transactions, setTransaction] = useState<Block[]>([]);
  useEffect(() => {
    getTransactions();
  }, [page, transaction]);
  useEffect(() => {
    getDataLength();
  }, []);
  const refresh = () => {
    getDataLength();
    getTransactions();
  };
  const getTransactions = async () => {
    let elements: Block[] = await Service.getTransactions(
      dataLength + 1 - page
    );
    setTransaction(elements);
  };
  const getDataLength = async () => {
    let dataLength = await Service.countDataLength();
    setDataLength(dataLength);
  };
  const rows = transactions.map((element) => (
    <tr key={element.id.toString()}>
      <td>{element.id.toString()}</td>
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
      <Button
        className={styles.text}
        variant="gradient"
        gradient={{ from: "teal", to: "blue", deg: 60 }}
        onClick={refresh}
      >
        Refresh
      </Button>
      <br />
      <br />
      <Table className={styles.text}>
        <thead>
          <tr>
            <th>ID</th>
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
  const [transaction, setTransaction] = useState(0);

  return (
    <Center>
      <div className={styles.container}>
        <Container>
          <h1 className={styles.heading}>Dashboard</h1>
          <GambleForm {...{ setTransaction, transaction }} />
          <TransactionComponent {...{ transaction }} />
        </Container>
      </div>
    </Center>
  );
};
export default Dashboard;
