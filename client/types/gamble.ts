export interface GambleData {
    win?: "true" | "false"
    amount: number
    newBalance: number
    error?: "Balance Error"    
}